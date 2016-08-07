use std::string::String as StdString;

use quickcheck::TestResult;
use viola::gap::String;
use viola::range::IntoRange;

use super::{SliceRange, Splice};

#[quickcheck]
fn from_std_into_std(std: StdString) -> bool {
    let buf = String::from(std.clone());
    std == buf.into(): StdString
}

#[quickcheck]
fn from_str_into_std(std: StdString) -> bool {
    let buf = String::from(std.as_str());
    std == buf.into(): StdString
}

#[quickcheck]
fn slice(std: StdString, range: SliceRange) -> TestResult {
    if !range.valid_str(&std) { return TestResult::discard(); }

    let buf = String::from(std.as_str());

    let std_slice = &std[range.into_range(0)];
    let buf_slice = buf.slice(range);

    TestResult::from_bool(std_slice.to_string() == buf_slice.to_string())
}

#[quickcheck]
fn slice_slice(std: StdString, first: SliceRange, second: SliceRange) -> TestResult {
    if !first.valid_str(&std) { return TestResult::discard(); }

    let buf = String::from(std.as_str());

    let std_first = &std[first.into_range(0)];
    let buf_first = buf.slice(first);

    if !second.valid_str(std_first) { return TestResult::discard(); }

    let std_second = &std_first[second.into_range(0)];
    let buf_second = buf_first.slice(second);

    TestResult::from_bool(std_second.to_string() == buf_second.to_string())
}

#[quickcheck]
fn slice_chars(std: StdString, range: SliceRange) -> TestResult {
    if !range.valid_str(&std) { return TestResult::discard(); }

    let buf = String::from(std.as_str());

    let std_chars = std[range.into_range(0)].chars();
    let buf_chars = buf.slice(range).chars();

    TestResult::from_bool(std_chars.eq(buf_chars))
}

#[quickcheck]
fn splice(init: StdString, dest: SliceRange, src: StdString) -> TestResult {
    if !dest.valid_str(&init) { return TestResult::discard(); }

    let mut std = init.clone();
    let mut buf = String::from(init);

    Splice::splice(&mut std, dest, &src);
    Splice::splice(&mut buf, dest, &src);

    TestResult::from_bool(std == buf.into(): StdString)
}

#[quickcheck]
fn splice_seq(init: StdString, splices: Vec<(SliceRange, StdString)>) -> TestResult {
    let mut std = init.clone();
    let mut buf = String::from(init);

    for (dest, src) in splices {
        if !dest.valid_str(&std) { return TestResult::discard(); }

        Splice::splice(&mut std, dest, &src);
        Splice::splice(&mut buf, dest, &src);
    }

    TestResult::from_bool(std == buf.into(): StdString)
}

#[quickcheck]
fn splice_slice(
    init: StdString,
    dest: SliceRange,
    src: StdString,
    slice: SliceRange,
) -> TestResult {
    if !dest.valid_str(&init) { return TestResult::discard(); }

    let mut std = init.clone();
    let mut buf = String::from(init);

    Splice::splice(&mut std, dest, &src);
    Splice::splice(&mut buf, dest, &src);

    if !slice.valid_str(&std) { return TestResult::discard(); }

    let std_slice = &std[slice.into_range(0)];
    let buf_slice = buf.slice(slice);

    TestResult::from_bool(std_slice.to_string() == buf_slice.to_string())
}
