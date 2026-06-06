use crate::rotation::Rotation;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
/// A singular piece type.
pub enum Piece {
    T = 0,
    I = 1,
    J = 2,
    L = 3,
    O = 4,
    S = 5,
    Z = 6,
}

impl Piece {
    pub const NB: usize = 7;
    pub const ALL: [Self; Self::NB] = [
        Self::T,
        Self::I,
        Self::J,
        Self::L,
        Self::O,
        Self::S,
        Self::Z,
    ];

    /// Canonical rotation for deduplication.
    /// - I, S, and Z have North/South and East/West symmetry and is only North or East.
    /// - O has North/East/South/West symmetry and is only North.
    /// - T, J, and L have no symmetries.
    pub const fn canonical(self, rotation: Rotation) -> Rotation {
        match self {
            Piece::I | Piece::S | Piece::Z => unsafe {
                std::mem::transmute::<u8, Rotation>((rotation as u8) & 1)
            },
            Piece::O => Rotation::North,
            Piece::T | Piece::J | Piece::L => rotation,
        }
    }

    /// Total number of canonical rotations for this piece.
    pub const fn canonical_rotations(self) -> usize {
        match self {
            Piece::I | Piece::S | Piece::Z => 2,
            Piece::O => 1,
            Piece::T | Piece::J | Piece::L => 4,
        }
    }
}
