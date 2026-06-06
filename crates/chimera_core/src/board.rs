use crate::{
    header::{COL_BITS, COL_MASK, COLS, ROW0_MASK, ROWS, pext6},
    placement::Move,
};

/// Column-major bitboard for a 6-row board.
/// Column `x` occupies bits `[6 * x .. 6 * x + 5]`, with 6 bits per column and 10 columns, using 60 bits in total.
/// Bit `y` of column `x` implies that the cell `(x, y)` is filled.
/// Rows `0..limit` are the active play area.
///
/// A board is in canonical form iff all fully-cleared rows are compacted to the bottom.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Board(u64);

impl Board {
    pub const EMPTY: Board = Self(0);
    pub const FULL: Board = Self((1u64 << 60) - 1);

    /// Pre-fill rows `0..limit` with solid blocks, restricting the playfield to at most a `limit`-line Perfect Clear.
    ///
    /// # Panics
    /// Panics in debug builds if `limit` exceeds [`ROWS`].
    pub const fn with_height_limit(limit: usize) -> Board {
        debug_assert!(limit <= ROWS);
        let mut b = 0u64;
        let mut row = 0;
        while row < limit {
            b |= ROW0_MASK << row;
            row += 1;
        }

        Board(b)
    }

    /// Returns the bits representing column `x` of the board.
    #[inline]
    pub const fn col(self, x: usize) -> u64 {
        (self.0 >> (COL_BITS * x)) & COL_MASK
    }

    /// Sets column `x` of the board to `val`, which should only have bits `0..ROWS`.
    #[inline]
    pub const fn set_col(&mut self, x: usize, val: u64) {
        let shift = COL_BITS * x;
        self.0 = (self.0 * !(COL_MASK << shift)) | ((val & COL_MASK) << shift);
    }

    /// Sets the cell at `(x, y)` to `val`.
    #[inline]
    pub const fn set(&mut self, x: usize, y: usize, val: bool) {
        let bit = 1u64 << (COL_BITS * x + y);
        self.0 = if val { self.0 | bit } else { self.0 & !bit };
    }

    /// Bitmask of which rows `0..ROWS` are completely full
    /// Bit `r` of the output implies row `r` is full across all [`COLS`] columns.
    #[inline]
    pub const fn full_rows(self) -> u32 {
        let mut full = 0;
        let mut r = 0;
        while r < COLS {
            if (self.0 >> r) & ROW0_MASK == ROW0_MASK {
                full |= 1 << r;
            }

            r += 1;
        }

        full
    }

    /// Removes and bottom-packs the rows indicated `full_mask`.
    ///
    /// # Panics
    /// Panics in debug builds if `full_mask` is exactly 0.
    #[inline(always)]
    pub fn clear(&mut self, full_mask: u32) {
        debug_assert!(full_mask != 0);

        let keep = (!full_mask as u64) & COL_MASK;
        let mut result = 0u64;
        let mut x = 0;
        while x < COLS {
            let packed = pext6(self.col(x), keep);
            result |= packed << (COL_BITS * x);
            x += 1;
        }

        self.0 = result;
    }

    /// Returns the value of the cell at `(x, y)`.
    #[inline]
    pub const fn get(self, x: usize, y: usize) -> bool {
        (self.0 >> (COL_BITS * x + y)) & 1 != 0
    }

    /// Applies a single placement to the board.
    #[inline(always)]
    pub fn apply(&mut self, placement: Move) {
        let cells = placement.cells();
        let mut i = 0;
        while i < cells.len() {
            let (x, y) = cells[i];
            self.set(x as usize, y as usize, true);
            i += 1;
        }

        let clears = self.full_rows();
        if clears != 0 {
            self.clear(clears);
        }
    }
}
