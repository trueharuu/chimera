use chimera_core::piece::Piece;

/// Which pieces remain in the current bag.
/// Bit `i` implies piece `i` has not yet been drawn.
/// When the state hits 0, it is reset to `0b1111111` as a new bag has started.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Bag(pub u8);

impl Bag {
    /// A full bag.
    pub const fn full() -> Self {
        Self(0b01111111)
    }

    /// The remaining pieces in the bag.
    pub fn remaining(self) -> impl Iterator<Item = Piece> {
        (0..7)
            .filter(move |&i| self.0 & (1 << i) != 0)
            .map(Piece::from_index)
    }

    /// Draw a single piece from the bag.
    pub const fn draw(self, p: Piece) -> Self {
        let next = self.0 & !(1 << p as u8);
        if next == 0 { Self::full() } else { Bag(next) }
    }

    /// The number of pieces remaining in the bag.
    pub const fn count(self) -> u32 {
        self.0.count_ones()
    }
}
