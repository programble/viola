use std::string::String as StdString;

use quickcheck::TestResult;
use viola::gap::String;

use super::{SliceRange, Splice};

#[quickcheck]
fn from_std_into_std(std: StdString) -> bool {
    std == String::from(std.clone()).into(): StdString
}

#[quickcheck]
fn from_str_into_std(std: StdString) -> bool {
    std == String::from(std.as_str()).into(): StdString
}

#[quickcheck]
fn slice(std: StdString, range: SliceRange) -> TestResult {
    if !std.is_char_boundary(range.0.start) || !std.is_char_boundary(range.0.end) {
        return TestResult::discard();
    }

    let buf = String::from(std.clone());

    let std_slice = &std[range.0.clone()];
    let buf_slice = buf.slice(range.0);

    TestResult::from_bool(std_slice.to_string() == buf_slice.to_string())
}

#[quickcheck]
fn slice_slice(std: StdString, range_a: SliceRange, range_b: SliceRange) -> TestResult {
    let discard = !std.is_char_boundary(range_a.0.start)
        || !std.is_char_boundary(range_a.0.end)
        || !std[range_a.0.clone()].is_char_boundary(range_b.0.start)
        || !std[range_a.0.clone()].is_char_boundary(range_b.0.end);
    if discard {
        return TestResult::discard();
    }

    let buf = String::from(std.clone());

    let std_slice = &std[range_a.0.clone()][range_b.0.clone()];
    let buf_slice = buf.slice(range_a.0).slice(range_b.0);

    TestResult::from_bool(std_slice.to_string() == buf_slice.to_string())
}

#[quickcheck]
fn slice_chars(std: StdString, range: SliceRange) -> TestResult {
    if !std.is_char_boundary(range.0.start) || !std.is_char_boundary(range.0.end) {
        return TestResult::discard();
    }

    let buf = String::from(std.clone());

    let std_chars = std[range.0.clone()].chars();
    let buf_chars = buf.slice(range.0).chars();

    TestResult::from_bool(std_chars.eq(buf_chars))
}

#[quickcheck]
fn splice(init: StdString, dest: SliceRange, src: StdString) -> TestResult {
    if !init.is_char_boundary(dest.0.start) || !init.is_char_boundary(dest.0.end) {
        return TestResult::discard();
    }

    let mut std = init.clone();
    let mut buf = String::from(init);

    Splice::splice(&mut std, dest.clone(), &src);
    Splice::splice(&mut buf, dest, &src);

    TestResult::from_bool(std == buf.into(): StdString)
}

#[quickcheck]
fn splice_seq(init: StdString, splices: Vec<(SliceRange, StdString)>) -> TestResult {
    let mut std = init.clone();
    let mut buf = String::from(init);

    for (dest, src) in splices {
        if !std.is_char_boundary(dest.0.start) || !std.is_char_boundary(dest.0.end) {
            return TestResult::discard();
        }

        Splice::splice(&mut std, dest.clone(), &src);
        Splice::splice(&mut buf, dest, &src);
    }

    TestResult::from_bool(std == buf.into(): StdString)
}

#[quickcheck]
fn splice_slice(
    init: StdString,
    dest: SliceRange,
    src: StdString,
    slice: SliceRange
) -> TestResult {
    if !init.is_char_boundary(dest.0.start) || !init.is_char_boundary(dest.0.end) {
        return TestResult::discard();
    }

    let mut std = init.clone();
    let mut buf = String::from(init);

    Splice::splice(&mut std, dest.clone(), &src);
    Splice::splice(&mut buf, dest, &src);

    if !std.is_char_boundary(slice.0.start) || !std.is_char_boundary(slice.0.end) {
        return TestResult::discard();
    }

    let std_slice = &std[slice.0.clone()];
    let buf_slice = buf.slice(slice.0);

    TestResult::from_bool(std_slice.to_string() == buf_slice.to_string())
}
