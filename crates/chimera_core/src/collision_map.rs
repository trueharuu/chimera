use crate::{
    board::Board,
    data::{PIECE_CELLS, x_range},
    header::{COL_BITS, COL_MASK, COLS},
    piece::Piece,
    rotation::Rotation,
};

/// Mapping from `(rotation, column)` to a bitmask of rows that collide with the piece in that rotation when placed at that column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CollisionMap(pub [Board; Rotation::NB]);

impl CollisionMap {
    /// Construct a collision map for the given board and piece.
    pub fn new(board: Board, piece: Piece) -> Self {
        let mut data = [Board::EMPTY; Rotation::NB];
        let canon_rots = piece.canonical_rotations();

        for rot_idx in 0..canon_rots {
            let rot = Rotation::from(rot_idx as u8);
            if !piece.is_canonical(rot) {
                continue;
            }

            let cells = PIECE_CELLS[piece as usize][rot_idx];
            let (x_min, x_max) = x_range(piece, rot);

            let mut combined = [0u64; COLS];

            // out-of-bounds columns
            for x in 0..x_min as usize {
                combined[x] = COL_MASK;
            }
            for x in x_max as usize + 1..COLS {
                combined[x] = COL_MASK;
            }

            // valid columns
            for x in x_min as usize..=x_max as usize {
                let col = cells.iter().fold(0u64, |acc, &(dx, dy)| {
                    let src_x = x as i32 - dx as i32;
                    if src_x < 0 || src_x >= COLS as i32 {
                        return acc | COL_MASK;
                    }
                    let src = board.0[src_x as usize];
                    acc | if dy == 0 {
                        src
                    } else if dy > 0 {
                        (src >> dy as u32) | (!0u64 << (64 - dy as u32))
                    } else {
                        (src << (-dy) as u32) | ((1u64 << (-dy) as u32) - 1)
                    }
                });
                combined[x] = col;
            }

            data[rot_idx].0 = combined;
        }

        Self(data)
    }
    
    /// Whether the piece in the given rotation would collide with the board if its center were at `(x, y)`.
    #[inline(always)]
    pub fn get(&self, x: usize, y: usize, rot: Rotation) -> bool {
        let bits = self.0[rot as usize].0[x];
        bits & (1u64 << y) != 0
    }

    /// Whether the piece in the given rotation is not "floating".
    #[inline(always)]
    pub fn landed(&self, x: usize, y: usize, rot: Rotation) -> bool {
        let bits = self.0[rot as usize].0[x];
        let mask = 1u64 << y;
        (bits & mask) == 0 && (y == 0 || (bits & (mask >> 1)) != 0)
    }

    /// Create a [`CollisionMap`] which only contains the landed states.
    #[inline(always)]
    pub fn landable(mut self) -> Self {
        let mut r = 0;
        while r < Rotation::NB {
            let mut new_cols = [0u64; COLS];
            let mut x = 0;
            while x < COLS {
                let mut y = 0;
                while y < COL_BITS {
                    let mask = 1u64 << y;
                    let bits = self.0[r].0[x];
                    if (bits & mask) == 0 && (y == 0 || (bits & (mask >> 1)) != 0) {
                        new_cols[x] |= mask;
                    }
                    y += 1;
                }
                x += 1;
            }

            let mut c = 0;
            while c < COLS {
                self.0[r].0[c] = new_cols[c] & COL_MASK;
                c += 1;
            }
            r += 1;
        }
        self
    }
}
