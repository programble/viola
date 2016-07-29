//! `Copy` ranges.

use std::ops::{self, Index, IndexMut};

/// A (half-open) range which is bounded at both ends: { x | start <= x < end }.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Range<Idx> {
    /// The lower bound of the range (inclusive).
    pub start: Idx,

    /// The upper bound of the range (exclusive).
    pub end: Idx,
}

impl<Idx: Copy> Range<Idx> {
    /// Returns the range before this.
    pub fn before(self) -> ops::RangeTo<Idx> {
        ..(self.start)
    }

    /// Returns the range after this.
    pub fn after(self) -> ops::RangeFrom<Idx> {
        (self.end)..
    }
}

impl<Idx> From<ops::Range<Idx>> for Range<Idx> {
    fn from(range: ops::Range<Idx>) -> Range<Idx> {
        Range {
            start: range.start,
            end: range.end,
        }
    }
}

impl<Idx> Into<ops::Range<Idx>> for Range<Idx> {
    fn into(self) -> ops::Range<Idx> {
        (self.start)..(self.end)
    }
}

impl Index<Range<usize>> for Vec<u8> {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &[u8] {
        &self[index.into(): ops::Range<usize>]
    }
}

impl IndexMut<Range<usize>> for Vec<u8> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut [u8] {
        &mut self[index.into(): ops::Range<usize>]
    }
}
