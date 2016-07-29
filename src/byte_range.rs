//! A `Copy` `Range<usize>` with `impl Index<ByteRange> for Vec<u8>`.

use std::ops::{Index, IndexMut, Range, RangeFrom, RangeTo};

/// A (half-open) range which is bounded at both ends: { x | start <= x < end }.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ByteRange {
    /// The lower bound of the range (inclusive).
    pub start: usize,

    /// The upper bound of the range (exclusive).
    pub end: usize,
}

impl ByteRange {
    /// Returns the length of the range.
    pub fn len(self) -> usize {
        self.end - self.start
    }

    /// Returns the range before this.
    pub fn before(self) -> RangeTo<usize> {
        ..(self.start)
    }

    /// Returns the range after this.
    pub fn after(self) -> RangeFrom<usize> {
        (self.end)..
    }
}

impl From<Range<usize>> for ByteRange {
    fn from(range: Range<usize>) -> Self {
        ByteRange {
            start: range.start,
            end: range.end,
        }
    }
}

impl Into<Range<usize>> for ByteRange {
    fn into(self) -> Range<usize> {
        (self.start)..(self.end)
    }
}

impl Index<ByteRange> for Vec<u8> {
    type Output = [u8];

    fn index(&self, index: ByteRange) -> &[u8] {
        self.index(index.into(): Range<usize>)
    }
}

impl IndexMut<ByteRange> for Vec<u8> {
    fn index_mut(&mut self, index: ByteRange) -> &mut [u8] {
        self.index_mut(index.into(): Range<usize>)
    }
}
