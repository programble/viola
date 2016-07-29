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

    /// Returns the two string slices before and after the gap.
    pub fn as_strs(&self) -> (&str, &str) {
        unsafe {
            (
                str::from_utf8_unchecked(&self.buf[self.gap.before()]),
                str::from_utf8_unchecked(&self.buf[self.gap.after()]),
            )
        }
    }

    fn gap_start(&self) -> *const u8 {
        unsafe { self.buf.as_ptr().offset(self.gap.start as isize) }
    }

    fn gap_end(&self) -> *const u8 {
        unsafe { self.buf.as_ptr().offset(self.gap.end as isize) }
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
            let src = self.gap_end();
            let dest = self.gap_start();
            unsafe { ptr::copy_nonoverlapping(src, dest as *mut u8, move_len); }

            self.gap.start += move_len;
            self.gap.end += move_len;

        } else if index < self.gap.start {
            let move_len = self.gap.start - index;
            unsafe {
                let src = self.buf.as_ptr().offset(index as isize);
                let dest = self.gap_end().offset(-(move_len as isize));
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

            self.buf.reserve_exact(additional);
            unsafe { self.buf.set_len(new_len); }

            // Move `after` to the end of the buffer.
            let after_len = old_len - self.gap.end;
            let after_src = self.gap_end();
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
        let (a, b) = self.as_strs();
        f.debug_tuple("GapBuffer")
            .field(&a)
            .field(&self.gap.len())
            .field(&b)
            .finish()
    }
}

impl Display for GapBuffer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        let (a, b) = self.as_strs();
        f.write_str(a)?;
        f.write_str(b)
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
