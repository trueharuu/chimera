use std::fmt::Display;

use crate::piece::Piece;

/// A static sequence of [`Piece`]s.
///
/// A piece can be stored within 3 bits, so a [`u64`] can safely store 21 pieces.
/// Pieces are ordered where the first `n` elements take up the first `3n` bits, and the first element is at the least significant bits.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Queue(u64);

impl Queue {
    /// An empty queue.
    pub const fn new() -> Self {
        Self(0)
    }

    /// A queue consisting of only `piece`.
    pub const fn one(piece: Piece) -> Self {
        Self(piece as u64 + 1)
    }

    /// Create a queue from a given `slice`.
    pub const fn from_slice(slice: &[Piece]) -> Self {
        let mut s = Self::new();
        let mut i = 0;
        while i < slice.len() {
            s.push_back(slice[i]);
            i += 1;
        }

        s
    }

    pub const fn len(&self) -> usize {
        if self.0 == 0 {
            0
        } else {
            (64 - self.0.leading_zeros()).div_ceil(3) as usize
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Add a singular [`Piece`] to the front of the queue.
    pub const fn push_front(&mut self, piece: Piece) {
        self.0 <<= 3;
        self.0 |= piece as u64 + 1;
    }

    /// Add a singular [`Piece`] to the end of the queue.
    pub const fn push_back(&mut self, piece: Piece) {
        self.0 |= (piece as u64 + 1) << (self.len() * 3)
    }

    /// Return the [`Piece`] located at index `n`, if any.
    pub const fn get(&self, n: usize) -> Option<Piece> {
        let bits = self.0 >> (n * 3) & 0b111;
        if bits == 0 {
            None
        } else {
            Some(unsafe { std::mem::transmute::<u8, Piece>(bits as u8 - 1) })
        }
    }

    /// The queue as a [`Vec<Piece>`].
    pub fn as_vec(&self) -> Vec<Piece> {
        let mut out = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            out.push(self.get(i).unwrap());
        }

        out
    }

    /// Return a [`Queue`] containing elements within indicies `start..end`.
    pub const fn slice(&self, start: usize, end: usize) -> Self {
        Self(self.0 >> (3 * start) & ((1 << (3 * (end - start))) - 1))
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Queue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.len() {
            write!(f, "{:?}", self.get(i).unwrap())?;
        }

        Ok(())
    }
}
