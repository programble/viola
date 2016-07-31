//! Gap buffer string.

use std::fmt::{Debug, Display, Formatter, Error as FmtError};
use std::ops::Range;
use std::str;

use gap::buffer::{GapBuffer, GapSlice};
use range::IntoRange;

/// Gap buffer string.
///
/// See the [`GapBuffer`](struct.GapBuffer.html) documentation for more information.
///
/// # Examples
///
/// ```
/// use viola::gap::string::GapString;
///
/// let mut buf = GapString::new();
///
/// // Inserting data.
/// buf.splice(.., "abcde"); // "abcde"
///
/// // Deleting data.
/// buf.splice(1..3, ""); // "ade"
///
/// // Replacing data.
/// buf.splice(..2, "hgf"); // "hgfe"
/// # assert_eq!("hgfe", Into::<String>::into(buf));
/// ```
pub struct GapString {
    buf: GapBuffer,
}

/// Slice of a gap buffer string.
#[derive(Debug)]
pub enum GapStr<'a> {
    /// Contiguous slice, i.e. completely either side of the gap.
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
            GapSlice::Contiguous(a) => unsafe {
                GapStr::Contiguous(str::from_utf8_unchecked(a))
            },
            GapSlice::Fragmented(a, b) => unsafe {
                GapStr::Fragmented(str::from_utf8_unchecked(a), str::from_utf8_unchecked(b))
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
            GapStr::Contiguous(a) => a.is_char_boundary(index),
            GapStr::Fragmented(a, _) if index < a.len() => a.is_char_boundary(index),
            GapStr::Fragmented(a, b) => b.is_char_boundary(index - a.len()),
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
        string.splice(0..0, slice);
        string
    }
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
            GapStr::Contiguous(b) if self.buf.gap_start_zero() => {
                f.debug_list().entry(&gap).entry(&b).finish()
            },
            GapStr::Contiguous(a) => {
                f.debug_list().entry(&a).entry(&gap).finish()
            },
            GapStr::Fragmented(a, b) => {
                f.debug_list().entry(&a).entry(&gap).entry(&b).finish()
            },
        }
    }
}

impl<'a> Display for GapStr<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match *self {
            GapStr::Contiguous(a) => f.write_str(a),
            GapStr::Fragmented(a, b) => {
                f.write_str(a)?;
                f.write_str(b)
            },
        }
    }
}

impl Display for GapString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.slice(..))
    }
}
