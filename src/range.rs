//! Range extensions.

use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

/// For converting different range types to `Range<usize>`.
pub trait IntoRange {
    /// Converts to a `Range` with a default end point.
    fn into_range(self, end: usize) -> Range<usize>;
}

impl IntoRange for Range<usize> {
    fn into_range(self, _end: usize) -> Range<usize> {
        self
    }
}

impl IntoRange for RangeFrom<usize> {
    fn into_range(self, end: usize) -> Range<usize> {
        self.start..end
    }
}

impl IntoRange for RangeFull {
    fn into_range(self, end: usize) -> Range<usize> {
        0..end
    }
}

impl IntoRange for RangeTo<usize> {
    fn into_range(self, _end: usize) -> Range<usize> {
        0..self.end
    }
}

/// Extensions for `Range<usize>`.
pub trait RangeExt {
    /// Returns the range before this.
    fn before(&self) -> RangeTo<usize>;

    /// Returns the range after this.
    fn after(&self) -> RangeFrom<usize>;

    /// Returns a range of length `len` with the same `start`.
    fn with_len(&self, len: usize) -> Self;

    /// Adds `len` to the starting point and end point.
    fn add(&self, len: usize) -> Self;

    /// Subtracts `len` from the starting point and end point.
    fn sub(&self, len: usize) -> Self;
}

impl RangeExt for Range<usize> {
    fn before(&self) -> RangeTo<usize> {
        ..self.start
    }

    fn after(&self) -> RangeFrom<usize> {
        self.end..
    }

    fn with_len(&self, len: usize) -> Self {
        self.start..(self.start + len)
    }

    fn add(&self, len: usize) -> Self {
        (self.start + len)..(self.end + len)
    }

    fn sub(&self, len: usize) -> Self {
        (self.start - len)..(self.end - len)
    }
}
