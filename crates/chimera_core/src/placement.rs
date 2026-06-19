use std::fmt::Debug;

use crate::{data::PIECE_CELLS, piece::Piece, rotation::Rotation, spin::Spin};

/// A piece placement.
///
/// `x` spans `0..10` (4 bits) whilst `y` spans `0..6` (3 bits).
/// `rot` spans `0..4` (2 bits) and `piece` spans `0..7` (3 bits).
/// `spin` spans `0..4` (2 bits).
///
/// In total, this fits in 14 bits so we can store with the following bit layout:
/// ```text
/// 00 XXXX YYY RR PPP SS
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move(u16);

impl Move {
    /// A null move. This is guaranteed to be invalid and takes the value `0xFFFF`.
    pub const fn null() -> Self {
        Self(0xFFFF)
    }

    /// Raw bits of the move.
    pub const fn bits(self) -> u16 {
        self.0
    }

    /// Create a move from raw bits. This is not guaranteed to be valid.
    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    /// Create a [`Move`] from its parts.
    pub const fn new(x: usize, y: usize, rot: Rotation, piece: Piece, spin: Spin) -> Self {
        debug_assert!(x < 10);
        debug_assert!(y < 6);
        Self(
            ((x as u16) << 10)
                | ((y as u16) << 7)
                | ((rot as u16) << 5)
                | ((piece as u16) << 2)
                | (spin as u16),
        )
    }

    /// The `x`-coordinate.
    pub const fn x(self) -> usize {
        ((self.0 >> 10) & 0b1111) as usize
    }

    /// The `y`-coordinate.
    pub const fn y(self) -> usize {
        ((self.0 >> 7) & 0b111) as usize
    }

    /// The rotation of the move.
    pub const fn rot(self) -> Rotation {
        unsafe { std::mem::transmute(((self.0 >> 5) & 0b11) as u8) }
    }

    /// The piece this move is done with.
    pub const fn piece(self) -> Piece {
        unsafe { std::mem::transmute(((self.0 >> 2) & 0b111) as u8) }
    }

    /// The spin state of the move.
    pub const fn spin(self) -> Spin {
        unsafe { std::mem::transmute((self.0 & 0b11) as u8) }
    }

    /// The cells this placement takes up.
    #[inline(always)]
    pub const fn cells(self) -> [(usize, usize); 4] {
        let x = self.x();
        let y = self.y();

        let o = PIECE_CELLS[self.piece() as usize][self.rot() as usize];

        [
            (
                x.wrapping_add_signed(o[0].0 as isize),
                y.wrapping_add_signed(o[0].1 as isize),
            ),
            (
                x.wrapping_add_signed(o[1].0 as isize),
                y.wrapping_add_signed(o[1].1 as isize),
            ),
            (
                x.wrapping_add_signed(o[2].0 as isize),
                y.wrapping_add_signed(o[2].1 as isize),
            ),
            (
                x.wrapping_add_signed(o[3].0 as isize),
                y.wrapping_add_signed(o[3].1 as isize),
            ),
        ]
    }

    /// Whether the move's rotation is the earliest congruent for its respective piece.
    ///
    /// - For [`Piece::T`], [`Piece::J`], and [`Piece::L`]: all 4 rotations are canonical.
    /// - For [`Piece::I`], [`Piece::S`], and [`Piece::Z`]: only [`Rotation::North`] and [`Rotation::East`] are canonical.
    /// - For [`Piece::O`]: only [`Rotation::North`] is canonical.
    pub const fn is_canonical(self) -> bool {
        match self.piece() {
            Piece::T | Piece::J | Piece::L => true,
            Piece::I | Piece::S | Piece::Z => {
                matches!(self.rot(), Rotation::North | Rotation::East)
            }
            Piece::O => matches!(self.rot(), Rotation::North),
            // _ => true,
        }
    }

    /// Returns a move which is congruent to this one, canonicalizing the rotation and moving the center.
    pub const fn canonicalize(self) -> Self {
        let (dx, dy) = self.piece().rotation_offset(self.rot());
        Self::new(
            self.x().wrapping_add_signed(dx as isize),
            self.y().wrapping_add_signed(dy as isize),
            self.piece().canonical(self.rot()),
            self.piece(),
            self.spin(),
        )
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Move")
            .field(&self.piece())
            .field(&self.x())
            .field(&self.y())
            .field(&self.rot())
            .field(&self.spin())
            .finish()
    }
}
