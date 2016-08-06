use std::string::String as StdString;

use super::{Buffer, Slice, String};

/// Uses the extra capacity as the gap.
impl From<Vec<u8>> for Buffer {
    fn from(mut buf: Vec<u8>) -> Self {
        let len = buf.len();
        let cap = buf.capacity();
        unsafe { buf.set_len(cap); }
        Buffer {
            buf: buf,
            gap: len..cap,
        }
    }
}

/// Moves the gap to the end as extra capacity.
impl Into<Vec<u8>> for Buffer {
    fn into(mut self) -> Vec<u8> {
        let len = self.len();
        self.move_gap_up(len);
        unsafe { self.buf.set_len(len); }
        self.buf
    }
}

impl<'a> From<&'a [u8]> for Buffer {
    fn from(slice: &'a [u8]) -> Self {
        let mut buffer = Buffer::new();
        buffer.splice(.., slice);
        buffer
    }
}

impl<'a> Into<Vec<u8>> for Slice<'a> {
    fn into(self) -> Vec<u8> {
        match self {
            Slice::Contiguous(back) => back.to_vec(),
            Slice::Fragmented(front, back) => {
                let mut vec = front.to_vec();
                vec.extend(back);
                vec
            },
        }
    }
}

/// Uses the extra capacity as the gap.
impl From<StdString> for String {
    fn from(string: StdString) -> Self {
        String { buf: Buffer::from(string.into_bytes()) }
    }
}

/// Moves the gap to the end as extra capacity.
impl Into<StdString> for String {
    fn into(self) -> StdString {
        unsafe { StdString::from_utf8_unchecked(self.buf.into()) }
    }
}

impl<'a> From<&'a str> for String {
    fn from(slice: &'a str) -> Self {
        let mut string = String::new();
        string.splice(.., slice);
        string
    }
}
