use std::ops::{Index, IndexMut};

use chimera_core::{
    board::Board,
    data::PIECE_CELLS,
    header::{COL_MASK, COLS},
    piece::Piece,
    rotation::Rotation,
    vector::SimdBoard,
};

/// Mapping from `(rotation, column)` to a bitmask of rows that collide with the piece in that rotation when placed at that column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CollisionMap(pub [Board; Rotation::NB]);

impl Index<Rotation> for CollisionMap {
    type Output = Board;

    #[inline(always)]
    fn index(&self, index: Rotation) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<Rotation> for CollisionMap {
    #[inline(always)]
    fn index_mut(&mut self, index: Rotation) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl CollisionMap {
    const fn extract_rot_data<const PIECE: u8>(
        rot: u8,
    ) -> (i8, i8, i8, i8, i8, i8, i8, i8, u64, u64, u64, u64) {
        let blocks = PIECE_CELLS[PIECE as usize][rot as usize];

        let mask0 = if blocks[0].1 < 0 {
            (1u64 << (-blocks[0].1) as usize) - 1
        } else {
            0
        };
        let mask1 = if blocks[1].1 < 0 {
            (1u64 << (-blocks[1].1) as usize) - 1
        } else {
            0
        };
        let mask2 = if blocks[2].1 < 0 {
            (1u64 << (-blocks[2].1) as usize) - 1
        } else {
            0
        };
        let mask3 = if blocks[3].1 < 0 {
            (1u64 << (-blocks[3].1) as usize) - 1
        } else {
            0
        };

        (
            blocks[0].0,
            blocks[0].1,
            blocks[1].0,
            blocks[1].1,
            blocks[2].0,
            blocks[2].1,
            blocks[3].0,
            blocks[3].1,
            mask0,
            mask1,
            mask2,
            mask3,
        )
    }

    /// Construct a collision map for the given board and piece.
    pub const fn new<const PIECE: u8>(board: Board) -> Self {
        let mut result = Self([Board::EMPTY; Rotation::NB]);

        const MAX_SIZE: usize = 3;
        let mut padded = [!0u64; MAX_SIZE + COLS + 2];
        padded[MAX_SIZE..MAX_SIZE + COLS].copy_from_slice(&board.0);

        macro_rules! make_rot {
            ($rot:expr) => {{
                let (dx0, dy0, dx1, dy1, dx2, dy2, dx3, dy3, mask0, mask1, mask2, mask3) =
                    const { Self::extract_rot_data::<PIECE>($rot) };

                let mut x = 0;
                while x < COLS as i8 {
                    unsafe {
                        let c0 = *padded.get_unchecked(MAX_SIZE + (x + dx0) as usize);
                        let c1 = *padded.get_unchecked(MAX_SIZE + (x + dx1) as usize);
                        let c2 = *padded.get_unchecked(MAX_SIZE + (x + dx2) as usize);
                        let c3 = *padded.get_unchecked(MAX_SIZE + (x + dx3) as usize);

                        let val0 = if dy0 >= 0 {
                            c0 >> dy0 as u32
                        } else {
                            (c0 << (-dy0) as u32) | mask0
                        };
                        let val1 = if dy1 >= 0 {
                            c1 >> dy1 as u32
                        } else {
                            (c1 << (-dy1) as u32) | mask1
                        };
                        let val2 = if dy2 >= 0 {
                            c2 >> dy2 as u32
                        } else {
                            (c2 << (-dy2) as u32) | mask2
                        };
                        let val3 = if dy3 >= 0 {
                            c3 >> dy3 as u32
                        } else {
                            (c3 << (-dy3) as u32) | mask3
                        };

                        result.0[$rot].0[x as usize] = val0 | val1 | val2 | val3;
                    }

                    x += 1;
                }
            }};
        }

        match Piece::from_index(PIECE).canonical_rotations() {
            1 => {
                make_rot!(0);
            }
            2 => {
                make_rot!(0);
                make_rot!(1);
            }
            4 => {
                make_rot!(0);
                make_rot!(1);
                make_rot!(2);
                make_rot!(3);
            }
            _ => unreachable!(),
        };

        result
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

#[derive(Clone, Copy, Debug)]
pub struct SimdCollisionMap([SimdBoard; 4]);

impl SimdCollisionMap {
    pub const EMPTY: Self = Self([SimdBoard::EMPTY; 4]);

    #[inline(always)]
    pub fn from_collision_map(cm: &CollisionMap) -> Self {
        Self([
            SimdBoard::from_board(cm.0[0]),
            SimdBoard::from_board(cm.0[1]),
            SimdBoard::from_board(cm.0[2]),
            SimdBoard::from_board(cm.0[3]),
        ])
    }

    #[inline(always)]
    pub fn to_collision_map(self) -> CollisionMap {
        CollisionMap([
            self.0[0].to_board(),
            self.0[1].to_board(),
            self.0[2].to_board(),
            self.0[3].to_board(),
        ])
    }

    /// Equivalent of `CollisionMap::landable()`: a position is landable iff the
    /// cell is usable but the cell one row above (i.e., `shift_up()`) is not.
    #[inline(always)]
    pub fn landable(self) -> Self {
        Self([
            self.0[0] & !self.0[0].shift_up(),
            self.0[1] & !self.0[1].shift_up(),
            self.0[2] & !self.0[2].shift_up(),
            self.0[3] & !self.0[3].shift_up(),
        ])
    }
}

impl std::ops::Index<usize> for SimdCollisionMap {
    type Output = SimdBoard;
    #[inline(always)]
    fn index(&self, i: usize) -> &SimdBoard {
        &self.0[i]
    }
}

impl std::ops::IndexMut<usize> for SimdCollisionMap {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut SimdBoard {
        &mut self.0[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chimera_core::piece::Piece;

    #[test]
    fn test_collision_map_basic() {
        let board = Board::EMPTY;
        let cmap = CollisionMap::new::<{ Piece::T as u8 }>(board);

        // T North at (5, 0) should be free
        assert!(cmap.free(5, 0, Rotation::North));

        // T South at (5, 0) should collide (mino at dy=-1)
        assert!(cmap.collides(5, 0, Rotation::South));

        // T South at (5, 1) should be free
        assert!(cmap.free(5, 1, Rotation::South));

        // Add block at (5, 0)
        let mut board = Board::EMPTY;
        board.set(5, 0);
        let cmap = CollisionMap::new::<{ Piece::T as u8 }>(board);

        // T North at (5, 0) should collide (mino at (0, 0))
        assert!(cmap.collides(5, 0, Rotation::North));
        // T North at (6, 0) should collide (mino at (-1, 0))
        assert!(cmap.collides(6, 0, Rotation::North));
        // T North at (4, 0) should collide (mino at (1, 0))
        assert!(cmap.collides(4, 0, Rotation::North));
        // T North at (5, -1) would collide, but let's check (5, 0) with dy=1 mino
        // T North at (5, 0) has mino at (0, 1), which would collide if block was at (5, 1)

        let mut board = Board::EMPTY;
        board.set(5, 1);
        let cmap = CollisionMap::new::<{ Piece::T as u8 }>(board);
        assert!(cmap.collides(5, 0, Rotation::North));
    }
}
