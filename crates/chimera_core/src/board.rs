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

    pub fn shift(self, dx: i32, dy: i32) -> Board {
        let mut result = [0u64; 10];

        for x in 0..10i32 {
            let src_x = x + dx; 

            if !(0..10).contains(&src_x) {
                result[x as usize] = u64::MAX;
            } else {
                let src_col = self.0[src_x as usize];
                result[x as usize] = shift_vertical(src_col, dy);
            }
        }

        Board(result)
    }

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

    /// Unset the cell at `(x, y)`.
    #[inline(always)]
    pub const fn unset(&mut self, x: usize, y: usize) {
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

    #[inline(always)]
    pub const fn apply(&mut self, c: Move) {
        self.set_many(&c.cells());
    }

    /// Bitmask of filled rows in the board.
    #[inline(always)]
    pub const fn filled_rows(self) -> u64 {
        let mut mask = self.0[0];
        let mut x = 1;
        while x < COLS {
            mask &= self.0[x];
            x += 1;
        }

        mask
    }

    /// Moves all completely-filled rows to the bottom.
    #[inline(always)]
    pub const fn clearshift(&mut self, mask: u64) {
        if mask == 0 {
            return;
        }

        let mut x = 0;
        while x < COLS {
            let col = self.0[x];

            let mut bottom = 0u64;
            let mut top = 0u64;

            let mut bottom_idx = 0;
            let mut top_idx = mask.count_ones() as usize;

            let mut y = 0;
            while y < 64 {
                let bit = (col >> y) & 1;

                if ((mask >> y) & 1) != 0 {
                    bottom |= bit << bottom_idx;
                    bottom_idx += 1;
                } else {
                    top |= bit << top_idx;
                    top_idx += 1;
                }

                y += 1;
            }

            self.0[x] = bottom | top;
            x += 1;
        }
    }

    /// Entirely remove all completely-filled rows and shift everything above down by `mask.count_ones()`.
    #[inline(always)]
    pub const fn clear(&mut self, mask: u64) {
        if mask == 0 {
            return;
        }

        let mut x = 0;
        while x < COLS {
            let col = self.0[x];
            let mut out = 0u64;
            let mut out_idx = 0;
            let mut y = 0;
            while y < 64 {
                if ((mask >> y) & 1) == 0 {
                    let bit = (col >> y) & 1;
                    out |= bit << out_idx;
                    out_idx += 1;
                }

                y += 1;
            }
            self.0[x] = out;
            x += 1;
        }
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

#[inline]
fn shift_vertical(col: u64, dy: i32) -> u64 {
    if dy == 0 {
        col
    } else if dy > 0 {
        // Shift up: upper rows move to lower bit positions
        // Fill top bits (high positions) with 1s (occupied)
        (col >> dy as u32) | (!0u64 << (64 - dy as u32))
    } else {
        // Shift down: lower rows move to higher bit positions
        // Fill bottom bits (low positions) with 1s (occupied)
        (col << (-dy) as u32) | ((1u64 << (-dy) as u32) - 1)
    }
}
