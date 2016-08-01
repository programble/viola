use quickcheck::TestResult;
use viola::gap::buffer::GapBuffer;

use super::{SliceRange, Splice};

#[quickcheck]
fn from_vec_into_vec(vec: Vec<u8>) -> bool {
    vec == GapBuffer::from(vec.clone()).into(): Vec<u8>
}

#[quickcheck]
fn from_slice_into_vec(vec: Vec<u8>) -> bool {
    vec == GapBuffer::from(vec.as_slice()).into(): Vec<u8>
}

#[quickcheck]
fn empty_splice(src: Vec<u8>) -> bool {
    let mut vec = Vec::new();
    let mut buf = GapBuffer::new();

    Splice::splice(&mut vec, SliceRange(0..0), &src);
    Splice::splice(&mut buf, SliceRange(0..0), &src);

    vec == buf.into(): Vec<u8>
}

#[quickcheck]
fn splice(init: Vec<u8>, dest: SliceRange, src: Vec<u8>) -> TestResult {
    if dest.0.start > init.len() || dest.0.end > init.len() {
        return TestResult::discard();
    }

    let mut vec = init.clone();
    let mut buf = GapBuffer::from(init);

    Splice::splice(&mut vec, dest.clone(), &src);
    Splice::splice(&mut buf, dest, &src);

    TestResult::from_bool(vec == buf.into(): Vec<u8>)
}

#[quickcheck]
fn splice_seq(init: Vec<u8>, splices: Vec<(SliceRange, Vec<u8>)>) -> TestResult {
    let mut vec = init.clone();
    let mut buf = GapBuffer::from(init);

    for (dest, src) in splices {
        if dest.0.start > vec.len() || dest.0.end > vec.len() {
            return TestResult::discard();
        }

        Splice::splice(&mut vec, dest.clone(), &src);
        Splice::splice(&mut buf, dest, &src);
    }

    TestResult::from_bool(vec == buf.into(): Vec<u8>)
}

#[quickcheck]
fn splice_slice(init: Vec<u8>, dest: SliceRange, src: Vec<u8>, slice: SliceRange) -> TestResult {
    if dest.0.start > init.len() || dest.0.end > init.len() {
        return TestResult::discard();
    }

    let mut vec = init.clone();
    let mut buf = GapBuffer::from(init);

    Splice::splice(&mut vec, dest.clone(), &src);
    Splice::splice(&mut buf, dest, &src);

    if slice.0.start > vec.len() || slice.0.end > vec.len() {
        return TestResult::discard();
    }

    TestResult::from_bool(buf.slice(slice.0.clone()).to_vec() == &vec[slice.0])
}
