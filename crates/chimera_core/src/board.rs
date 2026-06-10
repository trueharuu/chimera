use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

use crate::{
    header::{COL_BITS, COL_MASK, COLS, fill_high, fill_low},
    placement::Move,
};

/// Column-major board: array of column bitvectors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Board(pub [u64; COLS]);

impl Board {
    pub const EMPTY: Self = Self([0u64; COLS]);

    /// Whether the cell `(x, y)` is filled.
    #[inline(always)]
    pub const fn get(self, x: usize, y: usize) -> bool {
        (self.0[x] >> y) & 1 != 0
    }

    /// Set the cell at `(x, y)`.
    #[inline(always)]
    pub const fn set(&mut self, x: usize, y: usize) {
        debug_assert!(x < COLS && y < COL_BITS);

        self.0[x] |= 1u64 << y;
    }

    /// Clear the cell at `(x, y)`.
    #[inline(always)]
    pub const fn clear(&mut self, x: usize, y: usize) {
        debug_assert!(x < COLS && y < COL_BITS);

        self.0[x] &= !(1u64 << y);
    }

    /// Set of the cells described in `slice`. See [`Board::set`].
    #[inline(always)]
    pub const fn set_many(&mut self, slice: &[(usize, usize)]) {
        let mut i = 0;
        while i < slice.len() {
            let (x, y) = slice[i];
            self.set(x, y);
            i += 1;
        }
    }

    /// The bits representing column `x`.
    #[inline(always)]
    pub const fn col(self, x: usize) -> u64 {
        self.0[x]
    }

    /// Shift every column by `dy` rows (filling out-of-bounds with 1s, i.e. solid)
    /// and shift columns by `dx` (filling out-of-bounds columns with 0s,
    /// since the x_min/x_max guard handles those explicitly).
    #[inline(always)]
    pub fn shift(self, dx: i32, dy: i32) -> Board {
        self.shift_rows(dy).shift_cols(dx)
    }

    #[inline(always)]
    pub const fn shift_rows(self, dy: i32) -> Board {
        if dy == 0 {
            return self;
        }

        if dy > 0 {
            let s = dy as u32;
            let mut out = [0u64; COLS];
            let mut x = 0;
            while x < COLS {
                let shifted = (self.0[x] << s) & COL_MASK;
                out[x] = shifted | fill_low(s);
                x += 1;
            }
            Board(out)
        } else {
            let s = (-dy) as u32;
            let mut out = [0u64; COLS];
            let mut x = 0;
            while x < COLS {
                let shifted = (self.0[x] >> s) & COL_MASK;
                out[x] = shifted | fill_high(s);
                x += 1;
            }
            Board(out)
        }
    }

    #[inline(always)]
    pub fn shift_cols(self, dx: i32) -> Board {
        let mut out = [0; COLS];

        if dx > 0 {
            let shift = dx as usize;
            if shift < COLS {
                out[shift..].copy_from_slice(&self.0[..COLS - shift]);
            }
        } else if dx < 0 {
            let shift = (-dx) as usize;
            if shift < COLS {
                out[..COLS - shift].copy_from_slice(&self.0[shift..]);
            }
        } else {
            return self;
        }

        Board(out)
    }

    #[inline(always)]
    pub const fn apply(&mut self, c: Move) {
        self.set_many(&c.cells());
    }
}

impl BitOr for Board {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut out = [0u64; COLS];
        let mut x = 0;
        while x < COLS {
            out[x] = self.0[x] | rhs.0[x];
            x += 1;
        }
        Board(out)
    }
}

impl BitOrAssign for Board {
    fn bitor_assign(&mut self, rhs: Self) {
        let mut x = 0;
        while x < COLS {
            self.0[x] |= rhs.0[x];
            x += 1;
        }
    }
}

impl BitAnd for Board {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut out = [0u64; COLS];
        let mut x = 0;
        while x < COLS {
            out[x] = self.0[x] & rhs.0[x];
            x += 1;
        }
        Board(out)
    }
}

impl BitAndAssign for Board {
    fn bitand_assign(&mut self, rhs: Self) {
        let mut x = 0;
        while x < COLS {
            self.0[x] &= rhs.0[x];
            x += 1;
        }
    }
}

impl Not for Board {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        let mut x = 0;
        while x < COLS {
            self.0[x] = !self.0[x];
            x += 1;
        }

        self
    }
}
