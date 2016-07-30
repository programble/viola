//! Gap buffers.

use byte_range::ByteRange;

/// Gap buffer.
pub struct GapBuffer {
    buf: Vec<u8>,
    gap: ByteRange,
}

/// Gap buffer string.
pub struct GapString {
    buf: GapBuffer,
}

mod buffer;
mod string;
mod fmt;
