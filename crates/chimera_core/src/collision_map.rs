use crate::{
    board::Board,
    data::PIECE_CELLS,
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
        let mut states = [Board::EMPTY; Rotation::NB];
        let mut padded = [u64::MAX; 20];
        padded[5..15].copy_from_slice(&board.0);

        for rot in 0..Rotation::NB {
            let r = Rotation::from(rot as u8);
            if !piece.is_canonical(r) {
                let (dx, dy) = piece.rotation_offset(r);
                states[rot] = states[piece.canonical(r) as usize].shift(dx as i32, -dy as i32);
                continue;
            }
            let cells = PIECE_CELLS[piece as usize][rot];
            let (dx0, dy0) = (cells[0].0 as isize, cells[0].1 as i32);
            let (dx1, dy1) = (cells[1].0 as isize, cells[1].1 as i32);
            let (dx2, dy2) = (cells[2].0 as isize, cells[2].1 as i32);
            let (dx3, dy3) = (cells[3].0 as isize, cells[3].1 as i32);

            for x in 0..COLS {
                let c0 = padded[(5 + x as isize + dx0) as usize];
                let c1 = padded[(5 + x as isize + dx1) as usize];
                let c2 = padded[(5 + x as isize + dx2) as usize];
                let c3 = padded[(5 + x as isize + dx3) as usize];

                let mut col_mask = if dy0 >= 0 {
                    c0 >> dy0
                } else {
                    (c0 << -dy0) | ((1u64 << -dy0) - 1)
                };
                col_mask |= if dy1 >= 0 {
                    c1 >> dy1
                } else {
                    (c1 << -dy1) | ((1u64 << -dy1) - 1)
                };
                col_mask |= if dy2 >= 0 {
                    c2 >> dy2
                } else {
                    (c2 << -dy2) | ((1u64 << -dy2) - 1)
                };
                col_mask |= if dy3 >= 0 {
                    c3 >> dy3
                } else {
                    (c3 << -dy3) | ((1u64 << -dy3) - 1)
                };

                states[rot].0[x] = col_mask;
            }
        }

        Self(states)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collision_map_basic() {
        let board = Board::EMPTY;
        let cmap = CollisionMap::new(board, Piece::T);

        // T North at (5, 0) should be free
        assert!(cmap.free(5, 0, Rotation::North));

        // T South at (5, 0) should collide (mino at dy=-1)
        assert!(cmap.collides(5, 0, Rotation::South));

        // T South at (5, 1) should be free
        assert!(cmap.free(5, 1, Rotation::South));

        // Add block at (5, 0)
        let mut board = Board::EMPTY;
        board.set(5, 0);
        let cmap = CollisionMap::new(board, Piece::T);

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
        let cmap = CollisionMap::new(board, Piece::T);
        assert!(cmap.collides(5, 0, Rotation::North));
    }
}
