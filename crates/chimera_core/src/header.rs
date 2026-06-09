/// The maximum number of rows a board can have.
pub const ROWS: usize = 6;
/// The number of columns a board has.
pub const COLS: usize = 10;

pub const COL_BITS: usize = 6;
pub const COL_MASK: u64 = 0b111111;

pub const ROW_MASK: u64 = 0x0410410410410411;
pub const TOP_ROW_MASK: u64 = 0x0820820820820820;

pub const COL_MASK_ALL: u64 = {
    let mut m = 0;
    let mut x = 0;
    while x < COLS {
        m |= COL_MASK << (COL_BITS * x);
        x += 1;
    }
    m
};

#[inline(always)]
pub const fn idx(x: usize, y: usize) -> usize {
    COL_BITS * x + y
}

#[inline(always)]
pub const fn fill_low(s: u32) -> u64 {
    let col = (1u64 << s) - 1;
    let mut m = 0u64;
    let mut x = 0;
    while x < COLS {
        m |= col << (COL_BITS * x);
        x += 1;
    }
    m
}

#[inline(always)]
pub const fn fill_high(s: u32) -> u64 {
    let col = !((1u64 << (COL_BITS as u32 - s)) - 1) & COL_MASK;
    let mut m = 0u64;
    let mut x = 0;
    while x < COLS {
        m |= col << (COL_BITS * x);
        x += 1;
    }
    m
}