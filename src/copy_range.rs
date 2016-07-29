//! `Copy` ranges.

use std::ops::{self, Index, IndexMut};

/// A (half-open) range which is bounded at both ends: { x | start <= x < end }.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Range {
    /// The lower bound of the range (inclusive).
    pub start: usize,

    /// The upper bound of the range (exclusive).
    pub end: usize,
}

impl Range {
    /// Returns the length of the range.
    pub fn len(self) -> usize {
        self.end - self.start
    }

    /// Returns the range before this.
    pub fn before(self) -> ops::RangeTo<usize> {
        ..(self.start)
    }

    /// Returns the range after this.
    pub fn after(self) -> ops::RangeFrom<usize> {
        (self.end)..
    }

    /// Shrinks the range to the specified length.
    pub fn shrink(self, len: usize) -> Self {
        Range {
            start: self.start,
            end: self.start + len,
        }
    }
}

impl From<ops::Range<usize>> for Range {
    fn from(range: ops::Range<usize>) -> Range {
        Range {
            start: range.start,
            end: range.end,
        }
    }
}

impl Into<ops::Range<usize>> for Range {
    fn into(self) -> ops::Range<usize> {
        (self.start)..(self.end)
    }
}

impl Index<Range> for Vec<u8> {
    type Output = [u8];

    fn index(&self, index: Range) -> &[u8] {
        &self[index.into(): ops::Range<usize>]
    }
}

impl IndexMut<Range> for Vec<u8> {
    fn index_mut(&mut self, index: Range) -> &mut [u8] {
        &mut self[index.into(): ops::Range<usize>]
    }
}
