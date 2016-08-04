//! Gap buffer string.

use std::fmt::{Debug, Display, Formatter, Error as FmtError};
use std::ops::Range;
use std::str::{self, Chars};

use gap::{GapIter, IterState};
use gap::buffer::{GapBuffer, GapSlice};
use range::IntoRange;

/// Gap buffer string.
///
/// See the [`GapBuffer`](../buffer/struct.GapBuffer.html) documentation for more information.
///
/// # Examples
///
/// ```
/// use viola::gap::string::{GapString, GapStr};
///
/// let mut buf = GapString::new();
///
/// buf.splice(.., "abcde");
/// assert_eq!(GapStr::Contiguous("abcde"), buf.slice(..));
///
/// buf.splice(1..3, "");
/// assert_eq!(GapStr::Fragmented("a", "de"), buf.slice(..));
///
/// buf.splice(..2, "hgf");
/// assert_eq!(GapStr::Fragmented("hgf", "e"), buf.slice(..));
/// ```
pub struct GapString {
    buf: GapBuffer,
}

/// Slice of a gap buffer string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GapStr<'a> {
    /// Contiguous slice, i.e. entirely either side of the gap.
    Contiguous(&'a str),

    /// Fragmented slice, i.e. separated by the gap.
    Fragmented(&'a str, &'a str),
}

impl GapString {
    /// Creates an empty string without allocating.
    pub fn new() -> Self {
        GapString { buf: GapBuffer::new() }
    }

    /// Creates a string with a pre-allocated gap.
    pub fn with_gap(len: usize) -> Self {
        GapString { buf: GapBuffer::with_gap(len) }
    }

    /// Returns the length of the string.
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Returns a slice of the string.
    ///
    /// # Panics
    ///
    /// Panics if the starting point is greater than the end point, or if either point is not a
    /// char boundary.
    pub fn slice<R: IntoRange>(&self, range: R) -> GapStr {
        match self.buf.slice(range) {
            GapSlice::Contiguous(back) => unsafe {
                GapStr::Contiguous(str::from_utf8_unchecked(back))
            },
            GapSlice::Fragmented(front, back) => unsafe {
                GapStr::Fragmented(
                    str::from_utf8_unchecked(front),
                    str::from_utf8_unchecked(back),
                )
            },
        }
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
        assert!(self.is_char_boundary(dest.start), "dest start not char boundary");
        assert!(self.is_char_boundary(dest.end), "dest end not char boundary");
        self.buf.splice(dest, src.as_bytes())
    }

    fn is_char_boundary(&self, index: usize) -> bool {
        match self.slice(..) {
            GapStr::Contiguous(back) => back.is_char_boundary(index),
            GapStr::Fragmented(front, _) if index < front.len() => front.is_char_boundary(index),
            GapStr::Fragmented(front, back) => back.is_char_boundary(index - front.len()),
        }
    }
}

/// Uses the extra capacity as the gap.
impl From<String> for GapString {
    fn from(string: String) -> Self {
        GapString { buf: GapBuffer::from(string.into_bytes()) }
    }
}

/// Moves the gap to the end as extra capacity.
impl Into<String> for GapString {
    fn into(self) -> String {
        unsafe { String::from_utf8_unchecked(self.buf.into()) }
    }
}

impl<'a> From<&'a str> for GapString {
    fn from(slice: &'a str) -> Self {
        let mut string = GapString::new();
        string.splice(.., slice);
        string
    }
}

impl<'a> GapStr<'a> {
    /// Converts a string slice to a byte slice.
    pub fn as_bytes(self) -> GapSlice<'a> {
        match self {
            GapStr::Contiguous(back) => GapSlice::Contiguous(back.as_bytes()),
            GapStr::Fragmented(front, back) => {
                GapSlice::Fragmented(front.as_bytes(), back.as_bytes())
            },
        }
    }

    /// Returns an iterator over the chars of a slice.
    pub fn chars(self) -> GapIter<Chars<'a>> {
        match self {
            GapStr::Contiguous(back) => GapIter {
                front: None,
                back: back.chars(),
                state: IterState::Back,
            },
            GapStr::Fragmented(front, back) => GapIter {
                front: Some(front.chars()),
                back: back.chars(),
                state: IterState::Both,
            },
        }
    }

    // TODO: char_indices, which will require its own iterator.

    // TODO: lines, which will require its own iterator.
}

struct Gap(usize);

impl Debug for Gap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "[..{}..]", self.0)
    }
}

impl Debug for GapString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        let gap = Gap(self.buf.gap_len());
        match self.slice(..) {
            GapStr::Contiguous(back) if self.buf.gap_start_zero() => {
                f.debug_list().entry(&gap).entry(&back).finish()
            },
            GapStr::Contiguous(front) => {
                f.debug_list().entry(&front).entry(&gap).finish()
            },
            GapStr::Fragmented(front, back) => {
                f.debug_list().entry(&front).entry(&gap).entry(&back).finish()
            },
        }
    }
}

impl<'a> Display for GapStr<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match *self {
            GapStr::Contiguous(back) => f.write_str(back),
            GapStr::Fragmented(front, back) => {
                f.write_str(front)?;
                f.write_str(back)
            },
        }
    }
}

impl Display for GapString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.slice(..))
    }
}
