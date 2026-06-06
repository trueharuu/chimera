#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// A [`Piece`] orientation. It is assumed that all pieces spawn facing [`Rotation::North`].
///
/// [`Piece`]: crate::piece::Piece
pub enum Rotation {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Rotation {
    pub const NB: usize = 4;
    pub const ALL: [Self; Self::NB] = [Self::North, Self::East, Self::South, Self::West];
    /// The rotation with a clockwise movement applied.
    pub const fn cw(self) -> Self {
        unsafe { std::mem::transmute((self as u8 + 1) & 3) }
    }

    /// The rotation with a counter-clockwise movement applied.
    pub const fn ccw(self) -> Self {
        unsafe { std::mem::transmute((self as u8 + 3) & 3) }
    }

    /// The rotation with a 180-degree movement applied.
    pub const fn flip(self) -> Self {
        unsafe { std::mem::transmute((self as u8 + 2) & 3) }
    }

    pub const fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value & 3) }
    }

    /// The [`Direction`] to transition from this rotation to `other`.
    pub const fn transition(self, other: Self) -> Direction {
        match (self, other) {
            (Self::North, Self::North)
            | (Self::South, Self::South)
            | (Self::West, Self::West)
            | (Self::East, Self::East) => Direction::None,
            (Self::North, Self::South)
            | (Self::South, Self::North)
            | (Self::East, Self::West)
            | (Self::West, Self::East) => Direction::Flip,
            (Self::North, Self::East)
            | (Self::East, Self::South)
            | (Self::South, Self::West)
            | (Self::West, Self::North) => Direction::CW,
            (Self::North, Self::West)
            | (Self::East, Self::North)
            | (Self::South, Self::East)
            | (Self::West, Self::South) => Direction::CCW,
        }
    }
}

/// A change in rotation.
pub enum Direction {
    None,
    CW,
    CCW,
    Flip,
}
