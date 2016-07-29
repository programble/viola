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

    /// Returns the length of the buffer (excluding gap).
    pub fn len(&self) -> usize {
        self.buf.len() - self.gap.len()
    }

    /// Returns the length of the gap.
    pub fn gap_len(&self) -> usize {
        self.gap.len()
    }

    /// Returns the two byte slices before and after the gap.
    pub fn as_slices(&self) -> (&[u8], &[u8]) {
        (&self.buf[self.gap.before()], &self.buf[self.gap.after()])
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
            unimplemented!()
        }

        self.gap.start
    }
}
