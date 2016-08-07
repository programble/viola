use quickcheck::TestResult;
use viola::gap::Buffer;

use super::{SliceRange, Splice};

#[quickcheck]
fn from_vec_into_vec(vec: Vec<u8>) -> bool {
    vec == Buffer::from(vec.clone()).into(): Vec<u8>
}

#[quickcheck]
fn from_slice_into_vec(vec: Vec<u8>) -> bool {
    vec == Buffer::from(vec.as_slice()).into(): Vec<u8>
}

#[quickcheck]
fn slice(vec: Vec<u8>, range: SliceRange) -> TestResult {
    if range.0.end > vec.len() {
        return TestResult::discard()
    }

    let buf = Buffer::from(vec.clone());

    let vec_slice = &vec[range.0.clone()];
    let buf_slice = buf.slice(range.0);

    TestResult::from_bool(vec_slice.to_vec() == buf_slice.into(): Vec<u8>)
}

#[quickcheck]
fn slice_slice(vec: Vec<u8>, range_a: SliceRange, range_b: SliceRange) -> TestResult {
    if range_a.0.end > vec.len() || range_b.0.end > range_a.0.len() {
        return TestResult::discard()
    }

    let buf = Buffer::from(vec.clone());

    let vec_slice = &vec[range_a.0.clone()][range_b.0.clone()];
    let buf_slice = buf.slice(range_a.0).slice(range_b.0);

    TestResult::from_bool(vec_slice.to_vec() == buf_slice.into(): Vec<u8>)
}

#[quickcheck]
fn slice_iter(vec: Vec<u8>, range: SliceRange) -> TestResult {
    if range.0.end > vec.len() {
        return TestResult::discard();
    }

    let buf = Buffer::from(vec.clone());
    let vec_iter = vec[range.0.clone()].iter();
    let buf_iter = buf.slice(range.0).into_iter();

    TestResult::from_bool(vec_iter.eq(buf_iter))
}

#[quickcheck]
fn splice(init: Vec<u8>, dest: SliceRange, src: Vec<u8>) -> TestResult {
    if dest.0.end > init.len() {
        return TestResult::discard();
    }

    let mut vec = init.clone();
    let mut buf = Buffer::from(init);

    Splice::splice(&mut vec, dest.clone(), &src);
    Splice::splice(&mut buf, dest, &src);

    TestResult::from_bool(vec == buf.into(): Vec<u8>)
}

#[quickcheck]
fn splice_seq(init: Vec<u8>, splices: Vec<(SliceRange, Vec<u8>)>) -> TestResult {
    let mut vec = init.clone();
    let mut buf = Buffer::from(init);

    for (dest, src) in splices {
        if dest.0.end > vec.len() {
            return TestResult::discard();
        }

        Splice::splice(&mut vec, dest.clone(), &src);
        Splice::splice(&mut buf, dest, &src);
    }

    TestResult::from_bool(vec == buf.into(): Vec<u8>)
}

#[quickcheck]
fn splice_slice(init: Vec<u8>, dest: SliceRange, src: Vec<u8>, slice: SliceRange) -> TestResult {
    if dest.0.end > init.len() {
        return TestResult::discard();
    }

    let mut vec = init.clone();
    let mut buf = Buffer::from(init);

    Splice::splice(&mut vec, dest.clone(), &src);
    Splice::splice(&mut buf, dest, &src);

    if slice.0.end > vec.len() {
        return TestResult::discard();
    }

    let vec_slice = &vec[slice.0.clone()];
    let buf_slice = buf.slice(slice.0);

    TestResult::from_bool(vec_slice.to_vec() == buf_slice.into(): Vec<u8>)
}
