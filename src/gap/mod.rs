//! Gap buffers.
//!
//! A gap buffer is a buffer of two contiguous segments with a gap between them. Editing operations
//! move data between the two segments and write data into the gap. If the gap is filled by new
//! data, a new one is allocated of half the total size of the buffer.
//!
//! A slice of a gap buffer can be either contiguous or fragmented. A contiguous slice is entirely
//! either side of the gap, while a fragmented slice is divided by it.
//!
//! The gap buffer offers a single operation, splice, which both deletes and inserts data. These
//! operations are performed by moving, expanding, and shrinking the gap.
//!
//! # Examples
//!
//! TODO

mod buffer;
mod cmp;
mod convert;
mod fmt;
mod iter;
mod slice;
mod str;
mod string;

pub use self::buffer::Buffer;
pub use self::iter::{CharIndices, Iter};
pub use self::slice::Slice;
pub use self::str::Str;
pub use self::string::String;
