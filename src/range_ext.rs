//! Range extensions.

use std::ops::{Range, RangeFrom, RangeTo};

/// Extensions for `Range<usize>`.
pub trait RangeExt {
    /// Returns the range before this.
    fn before(&self) -> RangeTo<usize>;

    /// Returns the range after this.
    fn after(&self) -> RangeFrom<usize>;

    /// Returns a range of length `len` with the same `start`.
    fn resize_end(&self, len: usize) -> Self;

    /// Returns a range of length `len` with the same `end`.
    fn resize_start(&self, len: usize) -> Self;
}

impl RangeExt for Range<usize> {
    fn before(&self) -> RangeTo<usize> {
        ..self.start
    }

    fn after(&self) -> RangeFrom<usize> {
        self.end..
    }

    fn resize_end(&self, len: usize) -> Self {
        self.start..(self.start + len)
    }

    fn resize_start(&self, len: usize) -> Self {
        (self.end - len)..self.end
    }
}
