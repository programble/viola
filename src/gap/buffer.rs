//! Gap buffer.

use std::fmt::{Debug, Formatter, Error as FmtError};
use std::ops::Range;
use std::ptr;

use range::{IntoRange, RangeExt};

/// Gap buffer.
///
/// A buffer of two contiguous segments with a gap between them. Editing operations move data
/// between the two segments and write data into the gap. If the gap is filled by new data, a new
/// one is allocated of half the total size of the buffer.
///
/// The gap buffer offers a single operation, splice, which deletes and inserts data.
///
/// # Examples
///
/// ```
/// use viola::gap::buffer::GapBuffer;
///
/// let mut buf = GapBuffer::new();
///
/// // Inserting data.
/// buf.splice(.., &[1, 2, 3, 4, 5]); // [1, 2, 3, 4, 5]
///
/// // Deleting data.
/// buf.splice(1..3, &[]); // [1, 4, 5]
///
/// // Replacing data.
/// buf.splice(..2, &[8, 7, 6]); // [8, 7, 6, 5]
/// # assert_eq!(vec![8, 7, 6, 5], Into::<Vec<u8>>::into(buf));
/// ```
pub struct GapBuffer {
    buf: Vec<u8>,
    gap: Range<usize>,
}

/// Slice of a gap buffer.
#[derive(Debug)]
pub enum GapSlice<'a> {
    /// Contiguous slice, i.e. completely either side of the gap.
    Contiguous(&'a [u8]),

    /// Fragmented slice, i.e. separated by the gap.
    Fragmented(&'a [u8], &'a [u8]),
}

impl GapBuffer {
    /// Creates an empty gap buffer without allocating.
    ///
    /// A gap will be allocated when data is inserted.
    pub fn new() -> Self {
        GapBuffer {
            buf: Vec::new(),
            gap: 0..0,
        }
    }

    /// Creates a gap buffer with a pre-allocated gap.
    pub fn with_gap(len: usize) -> Self {
        let mut buffer = GapBuffer::new();
        buffer.resize_buf(len);
        buffer.gap = 0..len;
        buffer
    }

    /// Returns the length of the buffer, excluding the gap.
    pub fn len(&self) -> usize {
        self.buf.len() - self.gap.len()
    }

    /// Returns a slice of the buffer.
    ///
    /// # Panics
    ///
    /// Panics if the starting point is greater than the end point, or if either point is out of
    /// bounds.
    pub fn slice<R: IntoRange>(&self, range: R) -> GapSlice {
        let range = range.into_range(self.len());

        if range.start < self.gap.start && range.end <= self.gap.start {
            GapSlice::Contiguous(&self.buf[range])
        } else if range.start >= self.gap.start {
            GapSlice::Contiguous(&self.buf[range.add(self.gap.len())])
        } else {
            GapSlice::Fragmented(
                &self.buf[range.with_end(self.gap.start)],
                &self.buf[range.add(self.gap.len()).with_start(self.gap.end)],
            )
        }
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
        let dest = &mut self.buf[self.gap.with_len(src.len())];
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

impl<'a> From<&'a [u8]> for GapBuffer {
    fn from(slice: &'a [u8]) -> Self {
        let mut buffer = GapBuffer::new();
        buffer.splice(0..0, slice);
        buffer
    }
}

// Used by the GapString Debug implementation.
impl GapBuffer {
    pub(super) fn gap_len(&self) -> usize {
        self.gap.len()
    }

    pub(super) fn gap_start_zero(&self) -> bool {
        self.gap.start == 0
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
        let gap = Gap(self.gap.len());
        match self.slice(..) {
            GapSlice::Contiguous(b) if self.gap.start == 0 => {
                f.debug_list().entry(&gap).entries(b).finish()
            },
            GapSlice::Contiguous(a) => {
                f.debug_list().entries(a).entry(&gap).finish()
            },
            GapSlice::Fragmented(a, b) => {
                f.debug_list().entries(a).entry(&gap).entries(b).finish()
            },
        }
    }
}
