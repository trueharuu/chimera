use crate::{
    board::Board,
    data::{PIECE_CELLS, x_range},
    header::{COL_MASK, COLS},
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

        for rot_idx in 0..4 {
            let rot = Rotation::from(rot_idx as u8);

            // if !piece.is_canonical(rot) {
            //     let (dx, dy) = piece.rotation_offset(rot);
            //     data[rot_idx] = data[piece.canonical(rot) as usize].shift(dx as i32, dy as i32);
            //     continue;
            // }

            let cells = PIECE_CELLS[piece as usize][rot_idx];

            let mut collision = !board;

            for (dx, dy) in cells {
                let free_at_offset = !board.shift(dx as i32, dy as i32);
                collision &= free_at_offset;
            }

            data[rot_idx] = !collision;
        }

        Self(data)
    }

    /// Whether the piece in the given rotation would collide with the board if its center were at `(x, y)`.
    #[inline(always)]
    pub const fn collides(&self, x: usize, y: usize, rot: Rotation) -> bool {
        let bits = self.0[rot as usize].0[x];
        bits & (1u64 << y) != 0
    }

    /// Whether the piece in the given rotation is does not collide.
    #[inline(always)]
    pub const fn free(&self, x: usize, y: usize, rot: Rotation) -> bool {
        !self.collides(x, y, rot)
    }

    /// Whether the piece in the given rotation is not "floating".
    #[inline(always)]
    pub const fn landed(&self, x: usize, y: usize, rot: Rotation) -> bool {
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
                let bits = self.0[r].0[x] & COL_MASK;
                self.0[r].0[x] = (!bits) & ((bits << 1) | 1);
                x += 1;
            }

            r += 1;
        }

        self
    }
}
