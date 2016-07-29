//! UTF-8 gap buffer.

use std::fmt::{Debug, Display, Formatter, Error as FmtError};
use std::str;

use copy_range::Range;

/// UTF-8 gap buffer.
pub struct GapBuffer {
    buf: Vec<u8>,
    gap: Range,
}

const GAP_LEN: usize = 128; // TODO: Determine.

impl GapBuffer {
    /// Creates an empty gap buffer.
    pub fn new() -> Self {
        let mut buf = Vec::new();
        buf.reserve_exact(GAP_LEN);
        unsafe { buf.set_len(GAP_LEN); }

        GapBuffer {
            buf: buf,
            gap: Range::from(0..GAP_LEN),
        }
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

    /// Inserts text at the start of the gap.
    pub fn insert(&mut self, src: &str) {
        if src.len() < self.gap.len() {
            let dest = &mut self.buf[self.gap.shrink(src.len())];
            dest.copy_from_slice(src.as_bytes());
            self.gap.start += src.len();
        } else {
            unimplemented!()
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

impl<'a> From<&'a str> for GapBuffer {
    fn from(src: &'a str) -> GapBuffer {
        let mut buf = GapBuffer::new();
        buf.insert(src);
        buf
    }
}
