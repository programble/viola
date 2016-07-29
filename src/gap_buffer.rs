//! UTF-8 gap buffer.

use std::fmt::{Debug, Display, Formatter, Error as FmtError};
use std::ptr;
use std::str;

use copy_range::Range;

/// UTF-8 gap buffer.
pub struct GapBuffer {
    buf: Vec<u8>,
    gap: Range,
}

impl GapBuffer {
    /// Creates an empty gap buffer.
    pub fn new() -> Self {
        GapBuffer {
            buf: Vec::new(),
            gap: Range::from(0..0),
        }
    }

    /// Returns the length of the string.
    pub fn len(&self) -> usize {
        self.buf.len() - self.gap.len()
    }

    /// Returns the string slice before the gap.
    pub fn before(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.buf[self.gap.before()]) }
    }

    /// Returns the byte slice of the gap.
    pub fn gap(&self) -> &[u8] {
        &self.buf[self.gap]
    }

    /// Returns the string slice after the gap.
    pub fn after(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.buf[self.gap.after()]) }
    }

    /// Moves the gap so that it starts at `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    pub fn move_gap(&mut self, index: usize) {
        // TODO: Panic if index is not a char boundary.
        if index > self.gap.start {
            assert!(index <= self.len(), "gap index out of bounds");

            let move_len = index - self.gap.start;
            let src = self.after().as_ptr();
            let dest = self.gap().as_ptr();
            unsafe { ptr::copy_nonoverlapping(src, dest as *mut u8, move_len); }

            self.gap.start += move_len;
            self.gap.end += move_len;

        } else if index < self.gap.start {
            let move_len = self.gap.start - index;
            unsafe {
                let src = self.buf.as_ptr().offset(index as isize);
                let dest = self.after().as_ptr().offset(-(move_len as isize));
                ptr::copy_nonoverlapping(src, dest as *mut u8, move_len);
            }

            self.gap.start -= move_len;
            self.gap.end -= move_len;
        }
    }

    /// Inserts text at the start of the gap.
    pub fn insert(&mut self, src: &str) {
        if src.len() < self.gap.len() {
            let dest = &mut self.buf[self.gap.shrink(src.len())];
            dest.copy_from_slice(src.as_bytes());
            self.gap.start += src.len();

        } else {
            // Allocate additional space for `src` and a new gap.
            let old_len = self.buf.len();
            let gap_len = (old_len + src.len()) / 2;
            let additional = src.len() - self.gap.len() + gap_len;
            let new_len = old_len + additional;

            // TODO: Make sure this isn't UB for some reason.
            self.buf.reserve_exact(additional);
            unsafe { self.buf.set_len(new_len); }

            // Move `after` to the end of the buffer.
            let after_len = old_len - self.gap.end;
            let after_src = self.after().as_ptr();
            unsafe {
                let after_dest = after_src.offset(additional as isize);
                ptr::copy(after_src, after_dest as *mut u8, after_len);
            }

            // Copy in `src`.
            let dest = &mut self.buf[self.gap.expand(src.len())];
            dest.copy_from_slice(src.as_bytes());

            // Set gap to newly allocated gap.
            self.gap.start += dest.len();
            self.gap.end = new_len - after_len;
        }
    }
}

impl Debug for GapBuffer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.debug_tuple("GapBuffer")
            .field(&self.before())
            .field(&self.gap())
            .field(&self.after())
            .finish()
    }
}

impl Display for GapBuffer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.write_str(self.before())?;
        f.write_str(self.after())
    }
}

impl Default for GapBuffer {
    fn default() -> Self {
        GapBuffer::new()
    }
}

impl<'a> From<&'a str> for GapBuffer {
    fn from(src: &'a str) -> Self {
        let mut buf = GapBuffer::new();
        buf.insert(src);
        buf
    }
}
