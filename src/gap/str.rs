use std::cmp::PartialEq;
use std::str;

use range::IntoRange;
use super::Slice;

/// Slice of a gap buffer string.
#[derive(Debug, Clone, Copy)]
pub enum Str<'a> {
    /// Contiguous slice, i.e. entirely either side of the gap.
    Contiguous(&'a str),

    /// Fragmented slice, i.e. separated by the gap.
    Fragmented(&'a str, &'a str),
}

impl<'a> Str<'a> {
    /// Returns the length of the string slice.
    pub fn len(&self) -> usize {
        match *self {
            Str::Contiguous(back) => back.len(),
            Str::Fragmented(front, back) => front.len() + back.len(),
        }
    }

    /// Returns `true` if the slice contains no data.
    pub fn is_empty(&self) -> bool {
        match *self {
            Str::Contiguous(back) => back.is_empty(),
            Str::Fragmented(..) => false,
        }
    }

    /// Converts a string slice to a byte slice.
    pub fn as_bytes(&self) -> Slice<'a> {
        match *self {
            Str::Contiguous(back) => Slice::Contiguous(back.as_bytes()),
            Str::Fragmented(front, back) => Slice::Fragmented(front.as_bytes(), back.as_bytes()),
        }
    }

    /// Returns a sub-slice of the string slice.
    ///
    /// # Panics
    ///
    /// Panics if the starting point is greater than the end point, or if either point is not a
    /// char boundary.
    pub fn slice<R: IntoRange>(&self, range: R) -> Self {
        let range = range.into_range(self.len());
        assert!(self.is_char_boundary(range.start), "slice start not char boundary");
        assert!(self.is_char_boundary(range.end), "slice end not char boundary");
        match self.as_bytes().slice(range) {
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

    /// Checks that `index`-th byte lies at the start and/or end of a UTF-8 code point sequence.
    pub fn is_char_boundary(&self, index: usize) -> bool {
        match *self {
            Str::Contiguous(back) => back.is_char_boundary(index),
            Str::Fragmented(front, _) if index < front.len() => front.is_char_boundary(index),
            Str::Fragmented(front, back) => back.is_char_boundary(index - front.len()),
        }
    }
}

impl<'a, 'b> PartialEq<&'b str> for Str<'a> {
    fn eq(&self, other: &&str) -> bool {
        if self.len() != other.len() {
            return false;
        }

        match *self {
            Str::Contiguous(back) => {
                back == *other
            },
            Str::Fragmented(front, back) => {
                front == &other[..front.len()]
                    && back == &other[front.len()..]
            },
        }
    }
}
