use std::fmt::{Debug, Display, Formatter, Error as FmtError};
use std::ops::Range;
use std::str;

use super::buffer::GapBuffer;

/// Gap buffer string.
pub struct GapString {
    buf: GapBuffer,
}

impl GapString {
    /// Creates a new empty string without allocating.
    pub fn new() -> Self {
        GapString { buf: GapBuffer::new() }
    }

    /// Creates a string with an allocated gap of `len` bytes.
    pub fn with_gap(len: usize) -> Self {
        GapString { buf: GapBuffer::with_gap(len) }
    }

    /// Returns the length of the string in bytes.
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Returns the two string slices before and after the gap.
    pub fn as_strs(&self) -> (&str, &str) {
        let (a, b) = self.buf.as_slices();
        unsafe {
            (str::from_utf8_unchecked(a), str::from_utf8_unchecked(b))
        }
    }

    /// Replaces the substring at `dest` with the string `src`.
    ///
    /// Returns the range of the written bytes.
    ///
    /// # Panics
    ///
    /// - Panics if `dest` is out of bounds.
    /// - Panics if `dest` is not on char boundaries.
    pub fn splice(&mut self, dest: Range<usize>, src: &str) -> Range<usize> {
        {
            // TODO: Refactor.
            let (a, b) = self.as_strs();
            let start = if dest.start < a.len() {
                a.is_char_boundary(dest.start)
            } else {
                b.is_char_boundary(dest.start - a.len())
            };
            let end = if dest.end < a.len() {
                a.is_char_boundary(dest.end)
            } else {
                b.is_char_boundary(dest.end - a.len())
            };
            assert!(start, "dest start not char boundary");
            assert!(end, "dest end not char boundary");
        }
        self.buf.splice(dest, src.as_bytes())
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
        let (a, b) = self.as_strs();
        f.debug_list()
            .entry(&a)
            .entry(&Gap(self.buf.gap_len()))
            .entry(&b)
            .finish()
    }
}

impl Display for GapString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        let (a, b) =  self.as_strs();
        f.write_str(a)?;
        f.write_str(b)
    }
}
