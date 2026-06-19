use chimera_core::placement::Move;

/// Maximum amount of reachable moves for a single piece.
pub const MAX_MOVES: usize = 256;

/// A fixed-sized stack-allocated list of placements, up to [`MAX_MOVES`] in total.
pub struct MoveBuffer {
    data: [Move; MAX_MOVES],
    len: usize,
}

impl MoveBuffer {

    /// Create an empty buffer. The elements are filled with [`Move::null()`].
    #[inline]
    pub const fn new() -> Self {
        MoveBuffer {
            data: [Move::null(); MAX_MOVES],
            len: 0,
        }
    }

    /// Clears the buffer.
    #[inline]
    pub const fn clear(&mut self){
        self.len = 0;
    }

    /// Append a move to the end of the buffer.
    #[inline]
    pub const fn push(&mut self, m: Move) {
        debug_assert!(self.len < MAX_MOVES);
        self.data[self.len] = m;
        self.len += 1;
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn as_slice(&self) -> &[Move] {
        &self.data[..self.len]
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [Move] {
        &mut self.data[..self.len]
    }

    #[inline]
    pub const fn iter(&self) -> MoveIter<'_> {
        MoveIter::new(self)
    }

    #[inline]
    pub const fn iter_mut(&mut self) -> MoveIterMut<'_> {
        MoveIterMut::new(self)
    }

    #[inline]
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&Move) -> bool,
    {
        let mut dst = 0;
        for src in 0..self.len {
            if f(&self.data[src]) {
                self.data[dst] = self.data[src];
                dst += 1;
            }
        }
        self.len = dst;
    }    
}

impl Default for MoveBuffer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MoveIter<'a> {
    buf: &'a MoveBuffer,
    ptr: usize,
}

impl<'a> MoveIter<'a> {
    pub const fn new(buf: &'a MoveBuffer) -> Self {
        Self { buf, ptr: 0 }
    }
}

impl<'a> Iterator for MoveIter<'a> {
    type Item = &'a Move;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr >= self.buf.len() {
            return None;
        }

        let value = &self.buf.data[self.ptr];
        self.ptr += 1;
        Some(value)
    }
} 

pub struct MoveIterMut<'a> {
    buf: &'a mut MoveBuffer,
    ptr: usize,
}

impl<'a> MoveIterMut<'a> {
    pub const fn new(buf: &'a mut MoveBuffer) -> Self {
        Self { buf, ptr: 0 }
    }
}

impl<'a> Iterator for MoveIterMut<'a> {
    type Item = &'a mut Move;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr >= self.buf.len() {
            return None;
        }

        let ptr = &mut self.buf.data[self.ptr] as *mut Move;

        self.ptr += 1;

        // SAFETY: `self.ptr` is never the same across two calls,
        // and therefore will never have two mutable references to the same element.
        unsafe { Some(&mut *ptr) }
    }
} 