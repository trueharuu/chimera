use crate::{
    board::Board,
    data::{PIECE_CELLS, x_range},
    header::{COL_BITS, COL_MASK, COLS, ROW_MASK, idx},
    piece::Piece,
    rotation::Rotation,
};

/// Mapping from `(rotation, column)` to a bitmask of rows that collide with the piece in that rotation when placed at that column.
pub struct CollisionMap([u64; Rotation::NB]);

impl CollisionMap {
    /// Construct a collision map for the given board and piece.
    pub const fn new(board: Board, piece: Piece) -> Self {
        let mut data = [0u64; Rotation::NB];
        let canon_rots = piece.canonical_rotations();

        let mut rot_idx = 0;
        while rot_idx < canon_rots {
            let rot = Rotation::from(rot_idx as u8);
            if !piece.is_canonical(rot) {
                rot_idx += 1;
                continue;
            }

            let cells = PIECE_CELLS[piece as usize][rot_idx];
            let (x_min, x_max) = x_range(piece, rot);

            // OR together the board shifted by each cell offset
            let mut combined = 0u64;
            let mut i = 0;
            while i < cells.len() {
                let (dx, dy) = cells[i];
                combined |= board.shift(-dx as i32, -dy as i32).0;
                i += 1;
            }

            // Fill out-of-bounds columns with COL_MASK
            let mut x = 0;
            while x < COLS {
                if x < x_min as usize || x > x_max as usize {
                    combined |= (COL_MASK) << (COL_BITS * x);
                }
                x += 1;
            }

            data[rot_idx] = combined;
            rot_idx += 1;
        }

        Self(data)
    }

    /// Whether the piece in the given rotation would collide with the board if its center were at `(x, y)`.
    #[inline]
    pub fn get(&self, x: usize, y: usize, rot: Rotation) -> bool {
        self.0[rot as usize] & (1 << idx(x, y)) != 0
    }

    /// Whether the piece in the given rotation is not "floating".
    #[inline]
    pub fn landed(&self, x: usize, y: usize, rot: Rotation) -> bool {
        !self.get(x, y, rot) && (y == 0 || self.get(x, y - 1, rot))
    }

    /// Create a [`CollisionMap`] which only contains the landed states.
    #[inline]
    pub fn landable(mut self) -> Self {
        let mut r = 0;
        while r < Rotation::NB {
            let shifted = self.0[r] << 1;
            let cleared = shifted & !ROW_MASK;

            let result = cleared | ROW_MASK;
            self.0[r] = result & Board::FULL.0;
            r += 1;
        }
        self
    }
}
