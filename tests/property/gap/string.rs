use quickcheck::TestResult;
use viola::gap::string::GapString;

use super::{SliceRange, Splice};

#[quickcheck]
fn from_string_into_string(string: String) -> bool {
    string == GapString::from(string.clone()).into(): String
}

#[quickcheck]
fn from_str_into_string(string: String) -> bool {
    string == GapString::from(string.as_str()).into(): String
}

#[quickcheck]
fn empty_splice(src: String) -> bool {
    let mut string = String::new();
    let mut buffer = GapString::new();

    Splice::splice(&mut string, SliceRange(0..0), &src);
    Splice::splice(&mut buffer, SliceRange(0..0), &src);

    string == buffer.into(): String
}

#[quickcheck]
fn splice(init: String, dest: SliceRange, src: String) -> TestResult {
    if !init.is_char_boundary(dest.0.start) || !init.is_char_boundary(dest.0.end) {
        return TestResult::discard();
    }

    let mut string = init.clone();
    let mut buffer = GapString::from(init);

    Splice::splice(&mut string, dest.clone(), &src);
    Splice::splice(&mut buffer, dest, &src);

    TestResult::from_bool(string == buffer.into(): String)
}

#[quickcheck]
fn splice_seq(init: String, splices: Vec<(SliceRange, String)>) -> TestResult {
    let mut string = init.clone();
    let mut buffer = GapString::from(init);

    for (dest, src) in splices {
        if !string.is_char_boundary(dest.0.start) || !string.is_char_boundary(dest.0.end) {
            return TestResult::discard();
        }

        Splice::splice(&mut string, dest.clone(), &src);
        Splice::splice(&mut buffer, dest, &src);
    }

    TestResult::from_bool(string == buffer.into(): String)
}

#[quickcheck]
fn splice_slice(init: String, dest: SliceRange, src: String, slice: SliceRange) -> TestResult {
    if !init.is_char_boundary(dest.0.start) || !init.is_char_boundary(dest.0.end) {
        return TestResult::discard();
    }

    let mut string = init.clone();
    let mut buffer = GapString::from(init);

    Splice::splice(&mut string, dest.clone(), &src);
    Splice::splice(&mut buffer, dest, &src);

    if !string.is_char_boundary(slice.0.start) || !string.is_char_boundary(slice.0.end) {
        return TestResult::discard();
    }

    TestResult::from_bool(buffer.slice(slice.0.clone()).to_string() == &string[slice.0])
}
