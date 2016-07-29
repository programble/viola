use std::ptr;

use byte_range::ByteRange;

/// Gap buffer using `Vec<u8>`.
pub struct GapBuffer {
    buf: Vec<u8>,
    gap: ByteRange,
}

impl GapBuffer {
    /// Creates an empty gap buffer without allocating.
    pub fn new() -> Self {
        GapBuffer {
            buf: Vec::new(),
            gap: ByteRange::from(0..0),
        }
    }

    /// Creates a buffer with an allocated gap of `len` bytes.
    pub fn with_gap(len: usize) -> Self {
        let mut buffer = GapBuffer::new();
        buffer.resize_buf(len);
        buffer.gap.end = len;
        buffer
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

    /// Returns the length of the buffer (excluding gap).
    pub fn len(&self) -> usize {
        self.buf.len() - self.gap.len()
    }

    /// Returns the length of the gap.
    pub fn gap_len(&self) -> usize {
        self.gap.len()
    }

    /// Returns the index of the start of the gap.
    pub fn gap_index(&self) -> usize {
        self.gap.start
    }

    /// Returns the two byte slices before and after the gap.
    pub fn as_slices(&self) -> (&[u8], &[u8]) {
        (&self.buf[self.gap.before()], &self.buf[self.gap.after()])
    }

    /// Moves the gap so that it starts at `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    pub fn move_gap(&mut self, index: usize) {
        if index > self.gap.start {
            assert!(index <= self.len(), "gap index out of bounds");

            let move_len = index - self.gap.start;
            unsafe {
                ptr::copy(self.gap_end(), self.gap_start() as *mut u8, move_len);
            }

            self.gap.start += move_len;
            self.gap.end += move_len;

        } else if index < self.gap.start {
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
    }

    /// Inserts bytes at the start of the gap. If the bytes cannot fit in the current gap, a new
    /// one is allocated of half the total size of the buffer.
    ///
    /// Returns the new index of the start of the gap.
    pub fn insert(&mut self, src: &[u8]) -> usize {
        if src.len() < self.gap.len() {
            let dest = &mut self.buf[self.gap.resize_end(src.len())];
            dest.copy_from_slice(src);
            self.gap.start += src.len();

        } else {
            // Allocate additional space for `src` and a new gap.
            let old_len = self.buf.len();
            let gap_len = (self.len() + src.len()) / 2;
            let additional = src.len() - self.gap.len() + gap_len;
            self.resize_buf(additional);

            // Move data after gap to end of buffer.
            unsafe {
                ptr::copy(
                    self.gap_end(),
                    self.gap_end().offset(additional as isize) as *mut u8,
                    old_len - self.gap.end,
                );
            }

            // Copy in bytes.
            let dest = &mut self.buf[self.gap.resize_end(src.len())];
            dest.copy_from_slice(src);

            // Set newly allocated gap.
            self.gap.start += src.len();
            self.gap.end += additional;
        }

        self.gap.start
    }
}
