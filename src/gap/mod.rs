//! Gap buffers.

use std::ops::Range;

/// Gap buffer.
pub struct GapBuffer {
    buf: Vec<u8>,
    gap: Range<usize>,
}

/// Gap buffer string.
pub struct GapString {
    buf: GapBuffer,
}

mod buffer;
mod string;
mod fmt;
