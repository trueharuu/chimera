use chimera_core::{
    board::Board,
    collision_map::CollisionMap,
    data::kicks_for,
    header::{COL_BITS, COL_MASK, COLS},
    piece::Piece,
    placement::Move,
    rotation::Rotation,
    spin::{Spin, Spins},
};
use crate::buffer::MoveBuffer;
use crate::detect::{is_immobile, t_spin_from_corners};
// no heap allocations for dedupe

/// 'Global' mode movegen. No restrictions are made other than SRS.
///
/// Spin detection is dependent on the value of `spins`.
///
/// If [`Spins::None`], nothing is a spin.
///
/// If [`Spins::T`], a piece is a spin if:
/// - The piece is [`Piece::T`].
/// - The last input is a rotation.
/// - 3 of the 4 squares diagonally adjacent to the piece's center are occupied.
/// - A [`Spin::Full`] is emitted if there are two cells in the "front" corners.
/// - Otherwise, if there is only one cell in the front and two in the back, a [`Spin::Mini`] is emitted.
/// - Any spin that is the result of the last kick (TST kick) in a (counter)clockwise rotation is upgraded to a [`Spin::Full`].
///
/// If [`Spins::All`], then a piece is a spin if it is either:
/// - Immobile, and cannot move in any of the 4 directions
/// - A case where a [`Spin::Mini`] would be emitted under `A = 1`.
///
/// The "front" corners of a T piece are the ones adjacent to the "sticking out" mino of the piece.
/// The floor and walls of the board are considered to be occupied.
///
/// Movegen should output both sets of placements if available. If the same position is reachable with both a spin and without (i.e. T placement vs. T-spin-mini),
/// then they are considered distinct.
pub fn movegen(board: Board, piece: Piece, spins: Spins, out: &mut MoveBuffer) {
    // Reset output
    out.clear();

    let cm = CollisionMap::new(board, piece);

    // build usable and candidate (landable) maps per rotation.
    // CollisionMap stores canonical rotations; map canonical -> rotation using canonicalize offsets.
    let landable = cm.landable();
    let mut usable: [Board; Rotation::NB] = [Board::EMPTY; Rotation::NB];
    let mut candidates: [Board; Rotation::NB] = [Board::EMPTY; Rotation::NB];

    // compute per-rotation offsets to map canonical coordinates -> rotation coordinates
    let mid_x = (COLS / 2).min(4); // safe center
    let mid_y = (COL_BITS / 2).min(2);
    let mut offsets: [(i32, i32); Rotation::NB] = [(0, 0); Rotation::NB];
    for r in 0..Rotation::NB {
        let rot = Rotation::from(r as u8);
        let (delta_x, delta_y) = piece.rotation_offset(rot);
        offsets[r] = (delta_x as i32, delta_y as i32);
    }

    for r in 0..Rotation::NB {
        let rot = Rotation::from(r as u8);
        let rc = piece.canonical(rot) as usize;

        // start from canonical collision columns and shift into this rotation's frame
        let base_cols = cm.0[rc].0;
        let mut cols_u = [0u64; COLS];
        let mut cols_c = [0u64; COLS];

        let (off_x, off_y) = offsets[r];
        for x in 0..COLS {
            // Build a single-column board for shifting the canonical column
            let mut col_board = Board::EMPTY;
            col_board.0[x] = base_cols[x] & COL_MASK;
            let shifted = col_board.shift(off_x, off_y);
            cols_u[x] = (!shifted.0[x]) & COL_MASK;

            // candidates come from landable canonical map shifted similarly
            let mut col_cboard = Board::EMPTY;
            col_cboard.0[x] = landable.0[rc].0[x] & COL_MASK;
            let shifted_c = col_cboard.shift(off_x, off_y);
            cols_c[x] = shifted_c.0[x] & COL_MASK;
        }

        usable[r] = Board(cols_u);
        candidates[r] = Board(cols_c);
    }

    // Fast init (surface smear + quick tucks)
    let mut search: [Board; Rotation::NB] = [Board::EMPTY; Rotation::NB];
    for r in 0..Rotation::NB {
        // mask inversion to avoid flipping bits outside board height
        let mut surface_cols = [0u64; COLS];
        for x in 0..COLS {
            surface_cols[x] = (!usable[r].0[x]) & COL_MASK;
        }
        let mut surface = Board(surface_cols);

        // smear downward via per-column doubling shifts (logical right shift)
        for x in 0..COLS {
            let mut col = surface.0[x];
            for shift in [1usize, 2, 4, 8, 16] {
                if COL_BITS >= shift {
                    col |= col >> shift;
                }
            }
            surface.0[x] = col & COL_MASK;
        }

        // s = !surface but masked
        let mut s_cols = [0u64; COLS];
        for x in 0..COLS {
            s_cols[x] = (!surface.0[x]) & COL_MASK;
        }
        let mut s = Board(s_cols);

        s = s | ((s.shift(-1, 0) | s.shift(1, 0)) & usable[r]);
        s = s | ((s.shift(-1, 0) | s.shift(1, 0)) & usable[r]);
        search[r] = s;
    }

    // Unsearched = usable \\ search
    let mut unsearched: [Board; Rotation::NB] = [Board::EMPTY; Rotation::NB];
    for r in 0..Rotation::NB {
        unsearched[r] = usable[r] & !search[r];
    }

    // done flags
    let mut done = [false; Rotation::NB];
    for r in 0..Rotation::NB {
        done[r] = !search[r].0.iter().any(|&v| v != 0);
    }

    let board_any = |b: &Board| b.0.iter().any(|&v| v != 0);

    // dedupe moves using a fixed-size bitset over 14-bit Move keys (no heap alloc)
    const KEY_BITS: usize = 14;
    const KEY_SIZE: usize = 1 << KEY_BITS; // 16384
    const KEY_WORDS: usize = KEY_SIZE / 64; // 256
    let mut seen: [u64; KEY_WORDS] = [0u64; KEY_WORDS];

    // BFS
    while done.iter().any(|&d| !d) {
        for r in 0..Rotation::NB {
            if done[r] {
                continue;
            }
            done[r] = true;

            if !board_any(&search[r]) {
                continue;
            }

            // translations flood-fill
            loop {
                let temp =
                    (search[r].shift(-1, 0) | search[r].shift(1, 0) | search[r].shift(0, -1))
                        & unsearched[r];
                if !board_any(&temp) {
                    break;
                }
                search[r] |= temp;
                unsearched[r] &= !temp;
            }

            // collect landable placements from this rotation
            let land = search[r] & candidates[r];

            for x in 0..COLS {
                let mut col = land.0[x];
                while col != 0 {
                    let y = col.trailing_zeros() as usize;

                    let spin = match spins {
                        Spins::None => Spin::None,
                        Spins::T => {
                            if piece == Piece::T {
                                t_spin_from_corners(board, x, y, Rotation::from(r as u8), false)
                            } else {
                                Spin::None
                            }
                        }
                        Spins::All => {
                            if piece == Piece::T {
                                t_spin_from_corners(board, x, y, Rotation::from(r as u8), false)
                            } else if is_immobile(&cm, Rotation::from(r as u8), x, y) {
                                Spin::Mini
                            } else {
                                Spin::None
                            }
                        }
                    };

                    let m = Move::new(x, y, Rotation::from(r as u8), piece, spin).canonicalize();
                    let key = m.bits();

                    // check/set seen bit
                    let ki = key as usize;
                    let wi = ki >> 6;
                    let bit = 1u64 << (ki & 63);
                    if (seen[wi] & bit) == 0 {
                        seen[wi] |= bit;
                        out.push(Move::from_bits(key));
                    }

                    col &= col - 1;
                }
            }

            // rotation kicks
            if piece != Piece::O {
                let from_rot = Rotation::from(r as u8);
                for &to_rot in &[from_rot.cw(), from_rot.ccw(), from_rot.flip()] {
                    let kicks = kicks_for(piece, from_rot, to_rot);
                    if kicks.is_empty() {
                        continue;
                    }

                    let r1 = to_rot as usize;

                    let mut temp_src = search[r];
                    let mut result = Board::EMPTY;
                    if !kicks.is_empty() {
                        let last = kicks.len() - 1;
                        for (i, &(kx, ky)) in kicks.iter().enumerate() {
                            let target = temp_src.shift(kx as i32, ky as i32) & usable[r1];
                            result |= target;
                            if i < last {
                                let succ = target.shift(-(kx as i32), -(ky as i32));
                                temp_src &= !succ;
                            }
                        }
                    }

                    let new_positions = result & unsearched[r1];
                    if board_any(&new_positions) {
                        search[r1] |= new_positions;
                        unsearched[r1] &= !new_positions;
                        done[r1] = false;
                    }
                }
            }

            // clear frontier for this rotation
            search[r] = Board::EMPTY;
        }
    }

    // done
}
