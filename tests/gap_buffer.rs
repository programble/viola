#![feature(plugin, type_ascription)]
#![plugin(quickcheck_macros)]

extern crate quickcheck;
extern crate viola;

use std::ops::Range;

use quickcheck::TestResult;
use viola::gap::GapBuffer;

trait Splice {
    fn splice(&mut self, dest: Range<usize>, src: &[u8]);
}

impl Splice for GapBuffer {
    fn splice(&mut self, dest: Range<usize>, src: &[u8]) {
        self.splice(dest, src);
    }
}

impl Splice for Vec<u8> {
    fn splice(&mut self, dest: Range<usize>, src: &[u8]) {
        self.drain(dest.clone());

        // Inefficient, but works.
        for &byte in src.iter().rev() {
            self.insert(dest.start, byte);
        }
    }
}

fn valid_dest(dest: &Range<usize>, len: usize) -> bool {
    dest.start <= dest.end
        && dest.start <= len
        && dest.end <= len
}

#[quickcheck]
fn empty_one_splice(src: Vec<u8>) -> bool {
    let mut vec = Vec::new();
    let mut buf = GapBuffer::new();

    vec.splice(0..0, &src);
    buf.splice(0..0, &src);

    vec == buf.into(): Vec<u8>
}

#[quickcheck]
fn one_splice(init: Vec<u8>, dest: Range<usize>, src: Vec<u8>) -> TestResult {
    if !valid_dest(&dest, init.len()) {
        return TestResult::discard();
    }

    let mut vec = init.clone();
    let mut buf = GapBuffer::from(init);

    vec.splice(dest.clone(), &src);
    buf.splice(dest, &src);

    TestResult::from_bool(vec == buf.into(): Vec<u8>)
}

#[quickcheck]
fn multi_splice(init: Vec<u8>, splices: Vec<(Range<usize>, Vec<u8>)>) -> TestResult {
    let mut vec = init.clone();
    let mut buf = GapBuffer::from(init);

    for (dest, src) in splices {
        if !valid_dest(&dest, vec.len()) {
            return TestResult::discard();
        }

        vec.splice(dest.clone(), &src);
        buf.splice(dest, &src);
    }

    TestResult::from_bool(vec == buf.into(): Vec<u8>)
}
