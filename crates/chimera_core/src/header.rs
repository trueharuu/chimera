/// The maximum number of rows a board can have.
pub const ROWS: usize = 6;
/// The number of columns a board has.
pub const COLS: usize = 10;

pub const COL_MASK: u64 = 0b111111;
pub const COL_BITS: usize = 6;

/// Mask which selects bit `0` of every column, and represents a full row.
pub const ROW0_MASK: u64 = {
    let mut m = 0u64;
    let mut x = 0;
    while x < COLS {
        m |= 1u64 << (COL_BITS * x);
        x += 1;
    }
    m
};

/// Parallel bit extract. Extract bits of `val` at positions marked by `mask` and packs them to low bits.
/// Only operates on the lowest 6 bits.
#[inline(always)]
pub fn pext6(val: u64, mask: u64) -> u64 {
    #[cfg(target_feature = "bmi2")]
    // SAFETY: BMI2 guaranteed by `target_feature`.
    return unsafe { core::arch::x86_64::_pext_u64(val, mask) };

    #[cfg(not(target_feature = "bmi2"))]
    {
        let mut result = 0u64;
        let mut out = 0;
        let mut m = mask & COL_MASK;
        while m != 0 {
            let bit = m.trailing_zeros();
            result |= ((val >> bit) & 1) << out;
            out += 1;
            m &= m - 1;
        }

        result
    }
}