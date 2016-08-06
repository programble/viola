use std::string::String as StdString;

use quickcheck::TestResult;
use viola::gap::String;

use super::{SliceRange, Splice};

#[quickcheck]
fn from_string_into_string(string: StdString) -> bool {
    string == String::from(string.clone()).into(): StdString
}

#[quickcheck]
fn from_str_into_string(string: StdString) -> bool {
    string == String::from(string.as_str()).into(): StdString
}

#[quickcheck]
fn empty_splice(src: StdString) -> bool {
    let mut string = StdString::new();
    let mut buffer = String::new();

    Splice::splice(&mut string, SliceRange(0..0), &src);
    Splice::splice(&mut buffer, SliceRange(0..0), &src);

    string == buffer.into(): StdString
}

#[quickcheck]
fn splice(init: StdString, dest: SliceRange, src: StdString) -> TestResult {
    if !init.is_char_boundary(dest.0.start) || !init.is_char_boundary(dest.0.end) {
        return TestResult::discard();
    }

    let mut string = init.clone();
    let mut buffer = String::from(init);

    Splice::splice(&mut string, dest.clone(), &src);
    Splice::splice(&mut buffer, dest, &src);

    TestResult::from_bool(string == buffer.into(): StdString)
}

#[quickcheck]
fn splice_seq(init: StdString, splices: Vec<(SliceRange, StdString)>) -> TestResult {
    let mut string = init.clone();
    let mut buffer = String::from(init);

    for (dest, src) in splices {
        if !string.is_char_boundary(dest.0.start) || !string.is_char_boundary(dest.0.end) {
            return TestResult::discard();
        }

        Splice::splice(&mut string, dest.clone(), &src);
        Splice::splice(&mut buffer, dest, &src);
    }

    TestResult::from_bool(string == buffer.into(): StdString)
}

#[quickcheck]
fn splice_slice(init: StdString, dest: SliceRange, src: StdString, slice: SliceRange) -> TestResult {
    if !init.is_char_boundary(dest.0.start) || !init.is_char_boundary(dest.0.end) {
        return TestResult::discard();
    }

    let mut string = init.clone();
    let mut buffer = String::from(init);

    Splice::splice(&mut string, dest.clone(), &src);
    Splice::splice(&mut buffer, dest, &src);

    if !string.is_char_boundary(slice.0.start) || !string.is_char_boundary(slice.0.end) {
        return TestResult::discard();
    }

    TestResult::from_bool(buffer.slice(slice.0.clone()).to_string() == &string[slice.0])
}
