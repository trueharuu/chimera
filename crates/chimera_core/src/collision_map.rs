use crate::{
    board::Board,
    data::{PIECE_CELLS, x_range},
    header::{COL_MASK, COLS},
    placement::Move,
    spin::Spin,
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
        // First compute canonical rotation collision maps.
        let mut r = 0;
        while r < Rotation::NB {
            let rot = Rotation::from(r as u8);
            if piece.is_canonical(rot) {
                let cells = PIECE_CELLS[piece as usize][r];
                let (x_min, x_max) = x_range(piece, rot);

                // For each destination column x, OR together the source columns
                // shifted vertically. Horizontal shift is free — just index offset.
                let mut combined = Board::EMPTY;
                let mut x = x_min as usize;
                while x <= x_max as usize {
                    let mut col = 0u64;
                    let mut i = 0;
                    while i < cells.len() {
                        let (dx, dy) = cells[i];
                        let src_x = x as i32 + dx as i32;
                        if src_x < 0 || src_x >= COLS as i32 {
                            col |= COL_MASK;
                            i += 1;
                            continue;
                        }
                        // vertical shift: dy > 0 means piece cell is above pivot,
                        // so board content moves down in the collision column
                        let src_col = board.0[src_x as usize];
                        col |= if dy == 0 {
                            src_col
                        } else if dy > 0 {
                            (src_col >> dy as u32) | !(!0u64 >> dy as u32) // fill top bits
                        } else {
                            (src_col << (-dy) as u32) | ((1u64 << (-dy) as u32) - 1) // fill bottom bits
                        };
                        i += 1;
                    }
                    combined.0[x] = col;
                    x += 1;
                }

                // out-of-bounds columns are all solid
                let mut x = 0;
                while x < COLS {
                    if x < x_min as usize || x > x_max as usize {
                        combined.0[x] = COL_MASK;
                    }
                    x += 1;
                }

                data[r] = combined;
            }
            r += 1;
        }

        // Fill non-canonical rotations by shifting the canonical board into the rotation's frame.
        // We compute the offset by canonicalizing a sample placement and inverting that transform.
        let mid_x = (COLS / 2).min(4);
        let mid_y = (crate::header::COL_BITS / 2).min(2);
        let mut r2 = 0;
        while r2 < Rotation::NB {
            let rot = Rotation::from(r2 as u8);
            if !piece.is_canonical(rot) {
                // sample original -> canonical
                let sample = Move::new(mid_x, mid_y, rot, piece, Spin::None);
                let canon = sample.canonicalize();
                let delta_x = canon.x() as isize - mid_x as isize;
                let delta_y = canon.y() as isize - mid_y as isize;
                let off_x = -(delta_x as i32);
                let off_y = -(delta_y as i32);

                let rc = canon.rot() as usize;
                data[r2] = data[rc].shift(off_x, off_y);
            }
            r2 += 1;
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
    pub const fn landable(mut self) -> Self {
        let mut r = 0;
        while r < Rotation::NB {
            let mut x = 0;
            while x < COLS {
                let bits = self.0[r].0[x];
                self.0[r].0[x] = (!bits & ((bits << 1) | 1)) & COL_MASK;
                x += 1;
            }
            r += 1;
        }
        self
    }
}
