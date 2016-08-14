use std::string::String as StdString;

use quickcheck::TestResult;
use viola::gap::String;

use super::{SliceRange, Splice};

macro_rules! discard {
    ($range:expr, $slice:expr) => {
        if !$range.valid_str($slice) {
            return TestResult::discard();
        }
    }
}

#[quickcheck]
fn from_std(init: StdString) -> bool {
    let buf = String::from(init.clone());
    buf == init
}

#[quickcheck]
fn from_str(init: StdString) -> bool {
    let buf = String::from(&init[..]);
    buf == init
}

#[quickcheck]
fn into_std(init: StdString) -> bool {
    let buf = String::from(&init[..]);
    buf.into(): StdString == init
}

#[quickcheck]
fn len(init: StdString) -> bool {
    let buf = String::from(&init[..]);
    buf.len() == init.len()
}

#[quickcheck]
fn slice(init: StdString, range: SliceRange) -> TestResult {
    discard!(range, &init);
    let buf = String::from(&init[..]);
    TestResult::from_bool(buf.slice(range) == init[range])
}

#[quickcheck]
fn slice_slice(init: StdString, one: SliceRange, two: SliceRange) -> TestResult {
    discard!(one, &init);
    let buf = String::from(&init[..]);

    let std_one = &init[one];
    let buf_one = buf.slice(one);

    discard!(two, std_one);
    TestResult::from_bool(buf_one.slice(two) == std_one[two])
}

#[quickcheck]
fn slice_chars(init: StdString, range: SliceRange) -> TestResult {
    discard!(range, &init);
    let buf = String::from(&init[..]);
    TestResult::from_bool(buf.slice(range).chars().eq(init[range].chars()))
}

#[quickcheck]
fn slice_char_indices(init: StdString, range: SliceRange) -> TestResult {
    discard!(range, &init);
    let buf = String::from(&init[..]);
    TestResult::from_bool(buf.slice(range).char_indices().eq(init[range].char_indices()))
}

#[quickcheck]
fn slice_to_string(init: StdString, range: SliceRange) -> TestResult {
    discard!(range, &init);
    let buf = String::from(&init[..]);
    TestResult::from_bool(buf.slice(range).to_string() == &init[range])
}

#[quickcheck]
fn splice(init: StdString, dest: SliceRange, src: StdString) -> TestResult {
    discard!(dest, &init);
    let mut std = init.clone();
    let mut buf = String::from(&init[..]);

    Splice::splice(&mut std, dest, &src);
    Splice::splice(&mut buf, dest, &src);

    TestResult::from_bool(buf == std)
}

#[quickcheck]
fn splice_splice(
    init: StdString,
    one: (SliceRange, StdString),
    two: (SliceRange, StdString),
) -> TestResult {
    discard!(one.0, &init);
    let mut std = init.clone();
    let mut buf = String::from(&init[..]);

    Splice::splice(&mut std, one.0, &one.1);
    Splice::splice(&mut buf, one.0, &one.1);

    discard!(two.0, &std);
    Splice::splice(&mut std, two.0, &two.1);
    Splice::splice(&mut buf, two.0, &two.1);

    TestResult::from_bool(buf == std)
}

#[quickcheck]
fn splice_slice(
    init: StdString,
    dest: SliceRange,
    src: StdString,
    range: SliceRange,
) -> TestResult {
    discard!(dest, &init);
    let mut std = init.clone();
    let mut buf = String::from(init);

    Splice::splice(&mut std, dest, &src);
    Splice::splice(&mut buf, dest, &src);

    discard!(range, &std);
    TestResult::from_bool(buf.slice(range) == std[range])
}

#[quickcheck]
fn splice_slice_chars(
    init: StdString,
    dest: SliceRange,
    src: StdString,
    range: SliceRange,
) -> TestResult {
    discard!(dest, &init);
    let mut std = init.clone();
    let mut buf = String::from(init);

    Splice::splice(&mut std, dest, &src);
    Splice::splice(&mut buf, dest, &src);

    discard!(range, &std);
    TestResult::from_bool(buf.slice(range).chars().eq(std[range].chars()))
}

#[quickcheck]
fn splice_slice_char_indices(
    init: StdString,
    dest: SliceRange,
    src: StdString,
    range: SliceRange,
) -> TestResult {
    discard!(dest, &init);
    let mut std = init.clone();
    let mut buf = String::from(init);

    Splice::splice(&mut std, dest, &src);
    Splice::splice(&mut buf, dest, &src);

    discard!(range, &std);
    TestResult::from_bool(buf.slice(range).char_indices().eq(std[range].char_indices()))
}

#[quickcheck]
fn splice_slice_to_string(
    init: StdString,
    dest: SliceRange,
    src: StdString,
    range: SliceRange,
) -> TestResult {
    discard!(dest, &init);
    let mut std = init.clone();
    let mut buf = String::from(init);

    Splice::splice(&mut std, dest, &src);
    Splice::splice(&mut buf, dest, &src);

    discard!(range, &std);
    TestResult::from_bool(buf.slice(range).to_string() == &std[range])
}
