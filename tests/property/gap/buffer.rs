use quickcheck::TestResult;
use viola::gap::Buffer;

use super::{SliceRange, Splice};

macro_rules! discard {
    ($range:expr, $slice:expr) => {
        if !$range.valid_slice($slice) {
            return TestResult::discard();
        }
    }
}

#[quickcheck]
fn from_vec(init: Vec<u8>) -> bool {
    let buf = Buffer::from(init.clone());
    buf == init
}

#[quickcheck]
fn from_slice(init: Vec<u8>) -> bool {
    let buf = Buffer::from(&init[..]);
    buf == init
}

#[quickcheck]
fn into_vec(init: Vec<u8>) -> bool {
    let buf = Buffer::from(&init[..]);
    buf.into(): Vec<u8> == init
}

#[quickcheck]
fn len(init: Vec<u8>) -> bool {
    let buf = Buffer::from(&init[..]);
    buf.len() == init.len()
}

#[quickcheck]
fn slice(init: Vec<u8>, range: SliceRange) -> TestResult {
    discard!(range, &init);
    let buf = Buffer::from(&init[..]);
    TestResult::from_bool(buf.slice(range) == init[range])
}

#[quickcheck]
fn slice_slice(init: Vec<u8>, one: SliceRange, two: SliceRange) -> TestResult {
    discard!(one, &init);
    let buf = Buffer::from(&init[..]);

    let vec_one = &init[one];
    let buf_one = buf.slice(one);
    discard!(two, vec_one);

    TestResult::from_bool(buf_one.slice(two) == vec_one[two])
}

#[quickcheck]
fn slice_iter(init: Vec<u8>, range: SliceRange) -> TestResult {
    discard!(range, &init);
    let buf = Buffer::from(&init[..]);
    TestResult::from_bool(buf.slice(range).into_iter().eq(init[range].iter()))
}

#[quickcheck]
fn slice_into_vec(init: Vec<u8>, range: SliceRange) -> TestResult {
    discard!(range, &init);
    let buf = Buffer::from(&init[..]);
    TestResult::from_bool(buf.slice(range).into(): Vec<u8> == &init[range])
}

#[quickcheck]
fn splice(init: Vec<u8>, dest: SliceRange, src: Vec<u8>) -> TestResult {
    discard!(dest, &init);
    let mut vec = init.clone();
    let mut buf = Buffer::from(init);

    Splice::splice(&mut vec, dest, &src);
    Splice::splice(&mut buf, dest, &src);

    TestResult::from_bool(buf == vec)
}

#[quickcheck]
fn splice_splice(
    init: Vec<u8>,
    one: (SliceRange, Vec<u8>),
    two: (SliceRange, Vec<u8>),
) -> TestResult {
    discard!(one.0, &init);
    let mut vec = init.clone();
    let mut buf = Buffer::from(init);

    Splice::splice(&mut vec, one.0, &one.1);
    Splice::splice(&mut buf, one.0, &one.1);

    discard!(two.0, &vec);

    Splice::splice(&mut vec, two.0, &two.1);
    Splice::splice(&mut buf, two.0, &two.1);

    TestResult::from_bool(buf == vec)
}

#[quickcheck]
fn splice_slice(init: Vec<u8>, dest: SliceRange, src: Vec<u8>, range: SliceRange) -> TestResult {
    discard!(dest, &init);
    let mut vec = init.clone();
    let mut buf = Buffer::from(init);

    Splice::splice(&mut vec, dest, &src);
    Splice::splice(&mut buf, dest, &src);

    discard!(range, &vec);
    TestResult::from_bool(buf.slice(range) == vec[range])
}

#[quickcheck]
fn splice_slice_iter(
    init: Vec<u8>,
    dest: SliceRange,
    src: Vec<u8>,
    range: SliceRange,
) -> TestResult {
    discard!(dest, &init);
    let mut vec = init.clone();
    let mut buf = Buffer::from(init);

    Splice::splice(&mut vec, dest, &src);
    Splice::splice(&mut buf, dest, &src);

    discard!(range, &vec);
    TestResult::from_bool(buf.slice(range).into_iter().eq(vec[range].iter()))
}

#[quickcheck]
fn splice_slice_into_vec(
    init: Vec<u8>,
    dest: SliceRange,
    src: Vec<u8>,
    range: SliceRange,
) -> TestResult {
    discard!(dest, &init);
    let mut vec = init.clone();
    let mut buf = Buffer::from(init);

    Splice::splice(&mut vec, dest, &src);
    Splice::splice(&mut buf, dest, &src);

    discard!(range, &vec);
    TestResult::from_bool(buf.slice(range).into(): Vec<u8> == &vec[range])
}
