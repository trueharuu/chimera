use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

use crate::{
    header::{COL_BITS, COL_MASK, COL_MASK_ALL, COLS, fill_high, fill_low, idx},
    placement::Move,
};

/// Column-major bitboard for a 10x40 board.
/// Bit `y` of column `x` implies that the cell `(x, y)` is filled.
///
/// A board is in canonical form iff all fully-cleared rows are compacted to the bottom.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Board(pub u64);

impl Board {
    pub const EMPTY: Self = Self(0);
    pub const FULL: Self = Self(0xFFFFFFFFFFFFFFF);

    /// Whether the cell `(x, y)` is filled.
    #[inline]
    pub const fn get(self, x: usize, y: usize) -> bool {
        (self.0 >> idx(x, y) & 1) != 0
    }

    /// Set the cell at `(x, y)` to `val`.
    #[inline]
    pub const fn set(&mut self, x: usize, y: usize, val: bool) {
        debug_assert!(x < COLS && y < 64);

        if val {
            self.0 |= 1u64 << idx(x, y);
        } else {
            self.0 &= !(1u64 << idx(x, y));
        }
    }

    /// Set of the cells described in `slice`. See [`Board::set`].
    #[inline]
    pub const fn set_many(&mut self, slice: &[(usize, usize)], val: bool) {
        let mut i = 0;
        while i < slice.len() {
            let (x, y) = slice[i];
            self.set(x, y, val);
            i += 1;
        }
    }

    /// The bits representing column `x`.
    #[inline]
    pub const fn col(self, x: usize) -> u64 {
        self.0 >> (COL_BITS * x) & COL_MASK
    }

    /// Shift every column by `dy` rows (filling out-of-bounds with 1s, i.e. solid)
    /// and shift columns by `dx` (filling out-of-bounds columns with 0s,
    /// since the x_min/x_max guard handles those explicitly).
    pub const fn shift(self, dx: i32, dy: i32) -> Board {
        self.shift_rows(dy).shift_cols(dx)
    }

    pub const fn shift_rows(self, dy: i32) -> Board {
        if dy == 0 {
            return self;
        }
        if dy > 0 {
            let s = dy as u32;
            let shifted = (self.0 << s) & COL_MASK_ALL;
            Board(shifted | fill_low(s))
        } else {
            let s = (-dy) as u32;
            let shifted = (self.0 >> s) & COL_MASK_ALL;
            Board(shifted | fill_high(s))
        }
    }

    pub const fn shift_cols(self, dx: i32) -> Board {
        if dx == 0 {
            self
        } else if dx > 0 {
            Board((self.0 << (dx as u32 * COL_BITS as u32)) & COL_MASK_ALL)
        } else {
            Board((self.0 >> ((-dx) as u32 * COL_BITS as u32)) & COL_MASK_ALL)
        }
    }

    pub const fn apply(&mut self, c: Move) {
        self.set_many(&c.cells(), true);
    }
}

impl BitOr for Board {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Board {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitAnd for Board {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Board {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}
