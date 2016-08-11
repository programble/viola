use quickcheck::TestResult;
use viola::gap::Buffer;
use viola::range::IntoRange;

use super::{SliceRange, Splice};

#[quickcheck]
fn from_vec_into_vec(vec: Vec<u8>) -> bool {
    let buf = Buffer::from(vec.clone());
    vec == buf.into(): Vec<u8>
}

#[quickcheck]
fn from_slice_into_vec(vec: Vec<u8>) -> bool {
    let buf = Buffer::from(vec.as_slice());
    vec == buf.into(): Vec<u8>
}

#[quickcheck]
fn slice(vec: Vec<u8>, range: SliceRange) -> TestResult {
    if !range.valid_slice(&vec) { return TestResult::discard(); }

    let buf = Buffer::from(vec.as_slice());

    let vec_slice = &vec[range.into_range(0)];
    let buf_slice = buf.slice(range);

    TestResult::from_bool(buf_slice == vec_slice)
}

#[quickcheck]
fn slice_slice(vec: Vec<u8>, first: SliceRange, second: SliceRange) -> TestResult {
    if !first.valid_slice(&vec) { return TestResult::discard(); }

    let buf = Buffer::from(vec.as_slice());

    let vec_first = &vec[first.into_range(0)];
    let buf_first = buf.slice(first);

    if !second.valid_slice(vec_first) { return TestResult::discard(); }

    let vec_second = &vec_first[second.into_range(0)];
    let buf_second = buf_first.slice(second);

    TestResult::from_bool(buf_second == vec_second)
}

#[quickcheck]
fn slice_iter(vec: Vec<u8>, range: SliceRange) -> TestResult {
    if !range.valid_slice(&vec) { return TestResult::discard(); }

    let buf = Buffer::from(vec.as_slice());

    let vec_iter = vec[range.into_range(0)].iter();
    let buf_iter = buf.slice(range).into_iter();

    TestResult::from_bool(vec_iter.eq(buf_iter))
}

#[quickcheck]
fn splice(init: Vec<u8>, dest: SliceRange, src: Vec<u8>) -> TestResult {
    if !dest.valid_slice(&init) { return TestResult::discard(); }

    let mut vec = init.clone();
    let mut buf = Buffer::from(init);

    Splice::splice(&mut vec, dest, &src);
    Splice::splice(&mut buf, dest, &src);

    TestResult::from_bool(vec == buf.into(): Vec<u8>)
}

#[quickcheck]
fn splice_seq(init: Vec<u8>, splices: Vec<(SliceRange, Vec<u8>)>) -> TestResult {
    let mut vec = init.clone();
    let mut buf = Buffer::from(init);

    for (dest, src) in splices {
        if !dest.valid_slice(&vec) { return TestResult::discard(); }

        Splice::splice(&mut vec, dest, &src);
        Splice::splice(&mut buf, dest, &src);
    }

    TestResult::from_bool(vec == buf.into(): Vec<u8>)
}

#[quickcheck]
fn splice_slice(init: Vec<u8>, dest: SliceRange, src: Vec<u8>, slice: SliceRange) -> TestResult {
    if !dest.valid_slice(&init) { return TestResult::discard(); }

    let mut vec = init.clone();
    let mut buf = Buffer::from(init);

    Splice::splice(&mut vec, dest, &src);
    Splice::splice(&mut buf, dest, &src);

    if !slice.valid_slice(&vec) { return TestResult::discard(); }

    let vec_slice = &vec[slice.into_range(0)];
    let buf_slice = buf.slice(slice);

    TestResult::from_bool(buf_slice == vec_slice)
}
