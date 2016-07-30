use std::fmt::{Debug, Formatter, Error as FmtError};
use std::ops::Range;
use std::ptr;

use range_ext::RangeExt;

/// Gap buffer.
pub struct GapBuffer {
    buf: Vec<u8>,
    gap: Range<usize>,
}

impl GapBuffer {
    /// Creates an empty gap buffer without allocating.
    pub fn new() -> Self {
        GapBuffer {
            buf: Vec::new(),
            gap: 0..0,
        }
    }

    /// Creates a buffer with an allocated gap of `len` bytes.
    pub fn with_gap(len: usize) -> Self {
        let mut buffer = GapBuffer::new();
        buffer.resize_buf(len);
        buffer.gap = 0..len;
        buffer
    }

    /// Returns the length of the buffer (excluding gap).
    pub fn len(&self) -> usize {
        self.buf.len() - self.gap.len()
    }

    /// Returns the two byte slices before and after the gap.
    pub fn as_slices(&self) -> (&[u8], &[u8]) {
        (&self.buf[self.gap.before()], &self.buf[self.gap.after()])
    }

    /// Replaces the bytes at `dest` with the bytes in `src`.
    ///
    /// Returns the range of the written bytes.
    ///
    /// # Panics
    ///
    /// Panics if the starting point is greater than the end point, or if either points are out of
    /// bounds.
    pub fn splice(&mut self, dest: Range<usize>, src: &[u8]) -> Range<usize> {
        assert!(dest.start <= dest.end, "dest start greater than dest end");

        if dest.start > self.gap.start {
            assert!(dest.start <= self.len(), "dest start out of bounds");
            self.move_gap_up(dest.start);
        } else if dest.start < self.gap.start {
            self.move_gap_down(dest.start);
        }

        assert!(self.gap.end + dest.len() <= self.buf.len(), "dest end out of bounds");
        self.gap.end += dest.len();

        if src.len() >= self.gap.len() {
            self.resize_to_fit(src.len());
        }
        self.copy_into_gap(src);

        dest.resize_end(src.len())
    }

    fn resize_buf(&mut self, additional: usize) {
        let new_len = self.buf.len() + additional;
        self.buf.reserve_exact(additional);
        unsafe { self.buf.set_len(new_len); }
    }

    fn gap_start(&self) -> *const u8 {
        unsafe { self.buf.as_ptr().offset(self.gap.start as isize) }
    }

    fn gap_end(&self) -> *const u8 {
        unsafe { self.buf.as_ptr().offset(self.gap.end as isize) }
    }

    fn move_gap_up(&mut self, index: usize) {
        let move_len = index - self.gap.start;
        unsafe {
            ptr::copy(self.gap_end(), self.gap_start() as *mut u8, move_len);
        }
        self.gap.start += move_len;
        self.gap.end += move_len;
    }

    fn move_gap_down(&mut self, index: usize) {
        let move_len = self.gap.start - index;
        unsafe {
            ptr::copy(
                self.buf.as_ptr().offset(index as isize),
                self.gap_end().offset(-(move_len as isize)) as *mut u8,
                move_len,
            );
        }
        self.gap.start -= move_len;
        self.gap.end -= move_len;
    }

    fn resize_to_fit(&mut self, fit: usize) {
        // Allocate enough for `fit` and new gap.
        let old_len = self.buf.len();
        let gap_len = (self.len() + fit) / 2;
        let additional = fit - self.gap.len() + gap_len;
        self.resize_buf(additional);

        // Move data after gap to the end of the buffer. This is a bit inefficient since the Vec
        // has already copied this data once.
        unsafe {
            ptr::copy(
                self.gap_end(),
                self.gap_end().offset(additional as isize) as *mut u8,
                old_len - self.gap.end,
            );
        }

        self.gap.end += additional;
    }

    fn copy_into_gap(&mut self, src: &[u8]) {
        let dest = &mut self.buf[self.gap.resize_end(src.len())];
        dest.copy_from_slice(src);
        self.gap.start += src.len();
    }
}

/// Uses the extra capacity as the gap.
impl From<Vec<u8>> for GapBuffer {
    fn from(mut buf: Vec<u8>) -> Self {
        let len = buf.len();
        let cap = buf.capacity();
        unsafe { buf.set_len(cap); }
        GapBuffer {
            buf: buf,
            gap: len..cap,
        }
    }
}

/// Moves the gap to the end as extra capacity.
impl Into<Vec<u8>> for GapBuffer {
    fn into(mut self) -> Vec<u8> {
        let len = self.len();
        self.move_gap_up(len);
        let mut buf = self.buf;
        unsafe { buf.set_len(len); }
        buf
    }
}

impl GapBuffer {
    // Used by the GapString Debug implementation.
    #[doc(hidden)]
    pub fn gap_len(&self) -> usize {
        self.gap.len()
    }
}

struct Gap(usize);

impl Debug for Gap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "[..{}..]", self.0)
    }
}

impl Debug for GapBuffer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        let (a, b) = self.as_slices();
        f.debug_list()
            .entries(a)
            .entry(&Gap(self.gap.len()))
            .entries(b)
            .finish()
    }
}
