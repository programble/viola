use std::ops::Range;
use std::ptr;

use range::{IntoRange, RangeExt};
use super::Slice;

/// Gap buffer.
pub struct Buffer {
    pub(super) buf: Vec<u8>,
    pub(super) gap: Range<usize>,
}

impl Buffer {
    /// Creates an empty buffer without allocating.
    ///
    /// A gap will be allocated when data is inserted.
    pub fn new() -> Self {
        Buffer {
            buf: Vec::new(),
            gap: 0..0,
        }
    }

    /// Creates a buffer with a pre-allocated gap.
    pub fn with_gap(gap: usize) -> Self {
        let mut buffer = Self::new();
        buffer.resize_buf(gap);
        buffer.gap = 0..gap;
        buffer
    }

    /// Returns the length of the buffer, excluding the gap.
    pub fn len(&self) -> usize {
        self.buf.len() - self.gap.len()
    }

    /// Returns `true` if the buffer contains no data.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a slice containing the entire buffer.
    pub fn as_slice(&self) -> Slice {
        if self.gap.start == 0 {
            Slice::Contiguous(&self.buf[self.gap.end..])
        } else if self.gap.end == self.buf.len() {
            Slice::Contiguous(&self.buf[..self.gap.start])
        } else {
            Slice::Fragmented(&self.buf[..self.gap.start], &self.buf[self.gap.end..])
        }
    }

    /// Returns a slice of the buffer.
    pub fn slice<R: IntoRange>(&self, range: R) -> Slice {
        self.as_slice().slice(range)
    }

    /// Replaces a slice of bytes. Destination and source can be different lengths.
    ///
    /// Returns the range of written bytes.
    ///
    /// # Panics
    ///
    /// Panics if the starting point is greater than the end point, or if either point is out of
    /// bounds.
    pub fn splice<R: IntoRange>(&mut self, dest: R, src: &[u8]) -> Range<usize> {
        let dest = dest.into_range(self.len());
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

        dest.with_len(src.len())
    }

    // Reallocates the buffer and makes all bytes available.
    fn resize_buf(&mut self, additional: usize) {
        let new_len = self.buf.len() + additional;
        self.buf.reserve_exact(additional);
        unsafe { self.buf.set_len(new_len); }
    }

    // Returns a pointer to the start of the gap.
    fn gap_start(&self) -> *const u8 {
        unsafe { self.buf.as_ptr().offset(self.gap.start as isize) }
    }

    // Returns a pointer to the end of the gap.
    fn gap_end(&self) -> *const u8 {
        unsafe { self.buf.as_ptr().offset(self.gap.end as isize) }
    }

    // Moves the gap up (higher index).
    pub(super) fn move_gap_up(&mut self, index: usize) {
        let move_len = index - self.gap.start;
        unsafe {
            ptr::copy(self.gap_end(), self.gap_start() as *mut u8, move_len);
        }
        self.gap = self.gap.add(move_len);
    }

    // Moves the gap down (lower index).
    fn move_gap_down(&mut self, index: usize) {
        let move_len = self.gap.start - index;
        unsafe {
            ptr::copy(
                self.buf.as_ptr().offset(index as isize),
                self.gap_end().offset(-(move_len as isize)) as *mut u8,
                move_len,
            );
        }
        self.gap = self.gap.sub(move_len);
    }

    // Reallocate with enough capacity for `fit` and a new gap.
    fn resize_to_fit(&mut self, fit: usize) {
        let old_len = self.buf.len();
        let gap_len = (self.len() + fit) / 2;
        let additional = fit - self.gap.len() + gap_len;
        self.resize_buf(additional);

        // Move data after the gap to the end of the buffer. This is a bit inefficient since the
        // Vec has already copied this data once.
        unsafe {
            ptr::copy(
                self.gap_end(),
                self.gap_end().offset(additional as isize) as *mut u8,
                old_len - self.gap.end,
            );
        }

        self.gap.end += additional;
    }

    // Copies data into the gap.
    fn copy_into_gap(&mut self, src: &[u8]) {
        let dest = &mut self.buf[self.gap.with_len(src.len())];
        dest.copy_from_slice(src);
        self.gap.start += src.len();
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}
