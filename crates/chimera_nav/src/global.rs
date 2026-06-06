use chimera_core::{
    board::Board,
    collision_map::CollisionMap,
    data::kicks_for,
    header::{COL_MASK, COLS},
    piece::Piece,
    placement::Move,
    rotation::Rotation,
    spin::Spin,
};

use crate::buffer::MoveBuffer;

/// 'Global' mode movegen. No restrictions are made other than SRS.
pub fn movegen(board: Board, piece: Piece, out: &mut MoveBuffer) {
    let cm = CollisionMap::new(board, piece);

    // `reachable[rot][x]`: bitmask of anchor rows reachable at `(rot, x)`
    // bit `y` set implies the piece can be at anchor `(x, y)` in rotation `rot`
    let mut reachable = [[0u8; COLS]; Rotation::NB];

    for rot in 0..Rotation::NB {
        for x in 0..COLS {
            let cm_col = cm.get_col(Rotation::from(rot as u8), x);
            if cm_col == COL_MASK as u8 {
                // fully blocked column, no placements possible
                continue;
            }

            let mut surface = cm_col;
            surface |= surface << 1;
            surface |= surface << 2;
            surface |= surface << 4;

            // reachable = cells not covered AND free
            let free = !cm_col & COL_MASK as u8;
            reachable[rot][x] = !surface & free;
        }
    }

    // remaining: u64 bitmask where bit `(rot * COLS + x)` implies
    // `(rot, x)` has unexpanded reachable bits that may propagate to neighbors.
    // all positions are initially dirty.
    let mut remaining: u64 = 0;

    for rot in 0..Rotation::NB {
        for x in 0..COLS {
            if reachable[rot][x] != 0 {
                remaining |= 1u64 << (rot * COLS + x);
            }
        }
    }

    while remaining != 0 {
        let idx = remaining.trailing_zeros() as usize;
        remaining &= remaining - 1; // clear lowest set bit

        let rot = idx / COLS;
        let x = idx % COLS;
        let rot_e = Rotation::from(rot as u8);

        // softdrop: flood-fill downward within this column
        // propagate reachable bits downwards through free space until stable
        // at most `HEIGHT - 1` iterations for HEIGHT rows.
        // we do this before horizontal movement or rotations so neighbors can see the fully-settled column state.
        let free = !cm.get_col(rot_e, x) & COL_MASK as u8;
        let mut r = reachable[rot][x];

        loop {
            let next = r | ((r >> 1) & free);
            if next == r {
                break;
            }

            r = next;
        }
        reachable[rot][x] = r;

        // horizontal movements: propagate reachable rows to adjacent columns at the same rotation.
        // a row `y` at `(rot, x)` can shift to `(rot, x ± 1)` iff row `y` is free there.
        for &nx in &[x.wrapping_sub(1), x + 1] {
            if nx >= COLS {
                continue;
            }

            let nx_free = !cm.get_col(rot_e, nx) & COL_MASK as u8;
            let new_bits = reachable[rot][x] & nx_free & !reachable[rot][nx];

            if new_bits != 0 {
                reachable[rot][nx] |= new_bits;
                remaining |= 1u64 << (rot * COLS + nx);
            }
        }

        // rotation (cw, ccw, and flip)
        // for each srs kick: try to rotate the piece from `(rot, x, y)` to `(new_rot, x + kx, y + ky)`.
        // a kick succeeds if the destination is free. only the first valid kick per destination is used.
        // O pieces have no kicks so we can specialize against them.
        if piece != Piece::O {
            for new_rot_e in [rot_e.cw(), rot_e.ccw(), rot_e.flip()] {
                let new_rot = new_rot_e as usize;

                // for each kick, find which source rows can reach the destination
                // we process all kicks but stop per-source-row at first valid kick
                // we can accumulate bits that have already been claimed by an earlier kick so later kicks don't double count
                let mut already_claimed: u8 = 0;

                for &(kx, ky) in kicks_for(piece, rot_e, new_rot_e) {
                    // new anchor column after kick
                    let nx = x as i8 + kx;
                    if nx < 0 || nx as usize >= COLS {
                        continue;
                    }

                    let nx = nx as usize;

                    // new anchor row = `source_y + ky`
                    // we need to map each source row `y` to destination row `y + ky`
                    // this is just a bitshift on the reachable mask
                    let dest_free = !cm.get_col(new_rot_e, nx) & COL_MASK as u8;

                    // source rows that are reachable and not claimed by an earlier kick
                    let source = reachable[rot][x] & !already_claimed;
                    if source == 0 {
                        break;
                    }

                    // shift source rows by `ky` to get destination rows
                    let dest_candidates = if ky >= 0 {
                        let shifted = (source as u16) << (ky as u16);
                        (shifted & COL_MASK as u16) as u8
                    } else {
                        source >> ((-ky) as u8)
                    };

                    // destination rows that are actually free
                    let dest_valid = dest_candidates & dest_free;

                    if dest_valid != 0 {
                        let new_bits = dest_valid & !reachable[new_rot][nx];
                        if new_bits != 0 {
                            reachable[new_rot][nx] |= new_bits;
                            remaining |= 1u64 << (new_rot * COLS + nx);
                        }

                        // these source rows are claimed by this kick
                        // shift back to source-row space to mark as claimed
                        let claimed_sources = if ky >= 0 {
                            dest_valid >> (ky as u8)
                        } else {
                            ((dest_valid as u16) << ((-ky) as u16)) as u8
                        };

                        already_claimed |= claimed_sources;
                    }
                }
            }
        }
    }

    // collect canonical placements so we only output one of the 1-4 congruents, if any
    let mut canonical_seen = [[false; COLS]; Rotation::NB];

    for rot in 0..Rotation::NB {
        let rot_e = Rotation::from(rot as u8);
        let canon_rot = piece.canonical(rot_e) as usize;

        let mut landable_cols = 0u16;
        for x in 0..COLS {
            let land = reachable[rot][x] & cm.landable(rot_e, x);
            if land != 0 {
                landable_cols |= 1 << x;
            }
        }

        while landable_cols != 0 {
            let x = landable_cols.trailing_zeros() as usize;
            landable_cols &= landable_cols - 1;

            // under canonicalization, {I,S,Z}-{N,S} and {I,S,Z}-{E,W} are the same, so only output one of them
            if canonical_seen[canon_rot][x] {
                continue;
            }

            canonical_seen[canon_rot][x] = true;

            // the landing row is the lowest set bit of the landable mask
            let mut land = reachable[rot][x] & cm.landable(rot_e, x);
            while land != 0 {
                let y = land.trailing_zeros() as usize;
                land &= land - 1;

                // TODO: output spin state correctly.
                let placement = Move::new(x, y, Rotation::from(canon_rot as u8), piece, Spin::None);

                out.push(placement);
            }
        }
    }
}
