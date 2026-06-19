use std::simd::{num::SimdUint, simd_swizzle, u64x4};

use crate::board::Board;

#[derive(Clone, Copy, Debug)]
pub struct SimdBoard([u64x4; 3]);

impl SimdBoard {
    pub const EMPTY: Self = Self([u64x4::splat(0); 3]);

    /// Convert from a [`Board`].
    pub const fn from_board(board: Board) -> Self {
        Self([
            u64x4::from_array([board.0[0], board.0[1], board.0[2], board.0[3]]),
            u64x4::from_array([board.0[4], board.0[5], board.0[6], board.0[7]]),
            u64x4::from_array([board.0[8], board.0[9], 0, 0]),
        ])
    }

    /// Convert back to a [`Board`].
    pub fn to_board(self) -> Board {
        let a = self.0[0].to_array();
        let b = self.0[1].to_array();
        let c = self.0[2].to_array();
        Board([a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3], c[0], c[1]])
    }

    /// Shifts the entire board left by one column.
    pub fn shift_left(self) -> Self {
        let [c0, c1, c2] = self.0;
        let zero = u64x4::splat(0);

        let new_c0 = simd_swizzle!(c0, c1, [1, 2, 3, 4]);
        let new_c1 = simd_swizzle!(c1, c2, [1, 2, 3, 4]);
        let new_c2 = simd_swizzle!(c2, zero, [1, 4, 4, 4]);

        Self([new_c0, new_c1, new_c2])
    }

    /// Shifts the entire board right by one column.
    pub fn shift_right(self) -> Self {
        let [c0, c1, c2] = self.0;
        let zero = u64x4::splat(0);

        let new_c0 = simd_swizzle!(zero, c0, [0, 4, 5, 6]);
        let new_c1 = simd_swizzle!(c0, c1, [3, 4, 5, 6]);
        let new_c2 = simd_swizzle!(c1, c2, [3, 4, 5, 6]);
        Self([new_c0, new_c1, new_c2])
    }

    /// Shifts the entire board down by one row.
    pub fn shift_down(self) -> Self {
        let one = u64x4::splat(1);
        Self([self.0[0] >> one, (self.0[1] >> one), (self.0[2] >> one)])
    }

    /// Shifts the entire board up by one row.
    pub fn shift_up(self) -> Self {
        let one = u64x4::splat(1);
        Self([self.0[0] << one, (self.0[1] << one), (self.0[2] << one)])
    }

    /// Whether all 10 data lanes are exactly 0.
    pub fn is_empty(self) -> bool {
        (self.0[0].reduce_or() | self.0[1].reduce_or() | self.0[2].reduce_or()) == 0
    }
}

impl PartialEq for SimdBoard {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        ((self.0[0] ^ other.0[0]).reduce_or()
            | (self.0[1] ^ other.0[1]).reduce_or()
            | (self.0[2] ^ other.0[2]).reduce_or())
            == 0
    }
}

impl std::ops::BitOrAssign for SimdBoard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0[0] |= rhs.0[0];
        self.0[1] |= rhs.0[1];
        self.0[2] |= rhs.0[2];
    }
}

impl std::ops::BitOr for SimdBoard {
    type Output = Self;

    #[inline(always)]
    fn bitor(mut self, rhs: Self) -> Self {
        self |= rhs;
        self
    }
}

impl std::ops::BitAndAssign for SimdBoard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0[0] &= rhs.0[0];
        self.0[1] &= rhs.0[1];
        self.0[2] &= rhs.0[2];
    }
}

impl std::ops::BitAnd for SimdBoard {
    type Output = Self;

    #[inline(always)]
    fn bitand(mut self, rhs: Self) -> Self {
        self &= rhs;
        self
    }
}

impl std::ops::BitXorAssign for SimdBoard {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0[0] ^= rhs.0[0];
        self.0[1] ^= rhs.0[1];
        self.0[2] ^= rhs.0[2];
    }
}

impl std::ops::BitXor for SimdBoard {
    type Output = Self;

    #[inline(always)]
    fn bitxor(mut self, rhs: Self) -> Self {
        self ^= rhs;
        self
    }
}

pub const PAD_MASK: u64x4 = u64x4::from_array([!0, !0, 0, 0]);
impl std::ops::Not for SimdBoard {
    type Output = Self;


    #[inline(always)]
    fn not(self) -> Self {
        
        Self([!self.0[0], !self.0[1], !self.0[2] & PAD_MASK])
    }
}
