use std::ops::Range;
use std::str;

use range::IntoRange;
use super::{Buffer, Slice, Str};

/// Gap buffer UTF-8 string.
pub struct String {
    pub(super) buf: Buffer,
}

impl String {
    /// Creates an empty string without allocating.
    pub fn new() -> Self {
        String { buf: Buffer::new() }
    }

    /// Creates a string with a pre-allocated gap.
    pub fn with_gap(gap: usize) -> Self {
        String { buf: Buffer::with_gap(gap) }
    }

    /// Returns the length of the string, excluding the gap.
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Returns a string slice containing the entire string.
    pub fn as_str(&self) -> Str {
        match self.buf.as_slice() {
            Slice::Contiguous(back) => unsafe {
                Str::Contiguous(str::from_utf8_unchecked(back))
            },
            Slice::Fragmented(front, back) => unsafe {
                Str::Fragmented(
                    str::from_utf8_unchecked(front),
                    str::from_utf8_unchecked(back),
                )
            },
        }
    }

    /// Returns a slice of the string.
    pub fn slice<R: IntoRange>(&self, range: R) -> Str {
        self.as_str().slice(range)
    }

    /// Replaces a slice of the string. Destination and source can be different lengths.
    ///
    /// Returns the range of written bytes.
    ///
    /// # Panics
    ///
    /// Panics if the starting point is greater than the end point, or if either point is not a
    /// char boundary.
    pub fn splice<R: IntoRange>(&mut self, dest: R, src: &str) -> Range<usize> {
        let dest = dest.into_range(self.len());
        assert!(self.as_str().is_char_boundary(dest.start), "dest start not char boundary");
        assert!(self.as_str().is_char_boundary(dest.end), "dest end not char boundary");
        self.buf.splice(dest, src.as_bytes())
    }
}
