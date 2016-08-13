use std::cmp::PartialEq;
use std::string::String as StdString;

use super::{Buffer, Slice, Str, String};

impl<'a> PartialEq<[u8]> for Slice<'a> {
    fn eq(&self, other: &[u8]) -> bool {
        match *self {
            Slice::Contiguous(back) => {
                back == other
            },
            Slice::Fragmented(front, back) if self.len() == other.len() => {
                front == &other[..front.len()]
                    && back == &other[front.len()..]
            },
            _ => false,
        }
    }
}

// Necessary because only RHS is a reference.
impl<'a, 'b> PartialEq<&'b [u8]> for Slice<'a> {
    fn eq(&self, other: &&[u8]) -> bool {
        self == *other
    }
}

impl PartialEq<[u8]> for Buffer {
    fn eq(&self, other: &[u8]) -> bool {
        self.slice(..) == other
    }
}

impl PartialEq<Vec<u8>> for Buffer {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.slice(..) == other[..]
    }
}

impl<'a> PartialEq<str> for Str<'a> {
    fn eq(&self, other: &str) -> bool {
        match *self {
            Str::Contiguous(back) => {
                back == other
            },
            Str::Fragmented(front, back) if self.len() == other.len() => {
                front == &other[..front.len()]
                    && back == &other[front.len()..]
            },
            _ => false,
        }
    }
}

// Necessary because only RHS is a reference.
impl<'a, 'b> PartialEq<&'b str> for Str<'a> {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<str> for String {
    fn eq(&self, other: &str) -> bool {
        self.slice(..) == other
    }
}

impl PartialEq<StdString> for String {
    fn eq(&self, other: &StdString) -> bool {
        self.slice(..) == other[..]
    }
}
