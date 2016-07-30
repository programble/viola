use std::str;

use byte_range::ByteRange;

use super::buffer::GapBuffer;

/// Gap buffer UTF-8 string.
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

    /// Returns the length of the gap.
    pub fn gap_len(&self) -> usize {
        self.buf.gap_len()
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
    pub fn replace(&mut self, dest: ByteRange, src: &str) -> ByteRange {
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
        self.buf.replace(dest, src.as_bytes())
    }
}
