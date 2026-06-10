/// The maximum number of rows a board can have.
pub const ROWS: usize = 20;
/// The number of columns a board has.
pub const COLS: usize = 10;

pub const COL_BITS: usize = ROWS;
pub const COL_MASK: u64 = 0b111111;



#[inline(always)]
pub const fn fill_low(s: u32) -> u64 {
    if s == 0 {
        0u64
    } else {
        ((1u64 << s) - 1) & COL_MASK
    }
}

#[inline(always)]
pub const fn fill_high(s: u32) -> u64 {
    if s == 0 {
        0u64
    } else {
        (!((1u64 << (COL_BITS as u32 - s)) - 1)) & COL_MASK
    }
}
