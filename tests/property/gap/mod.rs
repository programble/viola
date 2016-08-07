mod buffer;
mod string;

use std::ops::Range;
use std::string::String as StdString;

use quickcheck::{Arbitrary, Gen};
use viola::gap::{Buffer, String};

// A `Range<usize>` where start <= end. Helps to generate more valid slice/splice operations.
#[derive(Debug, Clone)]
pub struct SliceRange(pub Range<usize>);

impl Arbitrary for SliceRange {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let start = usize::arbitrary(g);
        let size = g.size();
        let end = g.gen_range(start, start + size);
        SliceRange(start..end)
    }

    fn shrink(&self) -> Box<Iterator<Item = Self>> {
        box self.0.shrink().map(SliceRange)
    }
}

// Trait for dispatching to gap buffer splice and naive `Vec`/`StdString` implementations.
pub trait Splice<S: ?Sized> {
    fn splice(&mut self, dest: SliceRange, src: &S);
}

impl Splice<[u8]> for Buffer {
    fn splice(&mut self, dest: SliceRange, src: &[u8]) {
        self.splice(dest.0, src);
    }
}

impl Splice<[u8]> for Vec<u8> {
    fn splice(&mut self, dest: SliceRange, src: &[u8]) {
        self.drain(dest.0.clone());

        // Inefficient, but works.
        for &byte in src.iter().rev() {
            self.insert(dest.0.start, byte);
        }
    }
}

impl Splice<str> for String {
    fn splice(&mut self, dest: SliceRange, src: &str) {
        self.splice(dest.0, src);
    }
}

impl Splice<str> for StdString {
    fn splice(&mut self, dest: SliceRange, src: &str) {
        self.drain(dest.0.clone());
        self.insert_str(dest.0.start, src);
    }
}
