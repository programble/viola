mod buffer;
mod string;

use std::ops::{Index, Range};
use std::string::String as StdString;

use quickcheck::{Arbitrary, Gen};
use viola::gap::{Buffer, String};
use viola::range::IntoRange;

// A `Range<usize>` where start <= end and `Copy` is implemented. Helps to generate more valid
// slice/splice operations.
#[derive(Debug, Clone, Copy)]
struct SliceRange {
    start: usize,
    end: usize,
}

impl SliceRange {
    fn valid_slice(&self, slice: &[u8]) -> bool {
        self.end <= slice.len()
    }

    fn valid_str(&self, slice: &str) -> bool {
        slice.is_char_boundary(self.start) && slice.is_char_boundary(self.end)
    }
}

impl IntoRange for SliceRange {
    fn into_range(self, _len: usize) -> Range<usize> {
        self.start..self.end
    }
}

impl Index<SliceRange> for Vec<u8> {
    type Output = [u8];

    fn index(&self, index: SliceRange) -> &[u8] {
        &self[index.start..index.end]
    }
}

impl Index<SliceRange> for [u8] {
    type Output = [u8];

    fn index(&self, index: SliceRange) -> &[u8] {
        &self[index.start..index.end]
    }
}

impl Index<SliceRange> for StdString {
    type Output = str;

    fn index(&self, index: SliceRange) -> &str {
        &self[index.start..index.end]
    }
}

impl Index<SliceRange> for str {
    type Output = str;

    fn index(&self, index: SliceRange) -> &str {
        &self[index.start..index.end]
    }
}

impl Arbitrary for SliceRange {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let start = usize::arbitrary(g);
        let size = g.size();
        let end = g.gen_range(start, start + size);
        SliceRange {
            start: start,
            end: end,
        }
    }

    fn shrink(&self) -> Box<Iterator<Item = Self>> {
        box self.into_range(0).shrink().map(|range| {
            SliceRange {
                start: range.start,
                end: range.end,
            }
        })
    }
}

trait Splice<S: ?Sized> {
    fn splice(&mut self, dest: SliceRange, src: &S);
}

impl Splice<[u8]> for Buffer {
    fn splice(&mut self, dest: SliceRange, src: &[u8]) {
        self.splice(dest, src);
    }
}

impl Splice<str> for String {
    fn splice(&mut self, dest: SliceRange, src: &str) {
        self.splice(dest, src);
    }
}

impl Splice<[u8]> for Vec<u8> {
    fn splice(&mut self, dest: SliceRange, src: &[u8]) {
        self.drain(dest.start..dest.end);

        // Inefficient, but works. Where is insert_slice?
        for &byte in src.iter().rev() {
            self.insert(dest.start, byte);
        }
    }
}

impl Splice<str> for StdString {
    fn splice(&mut self, dest: SliceRange, src: &str) {
        self.drain(dest.start..dest.end);
        self.insert_str(dest.start, src);
    }
}
