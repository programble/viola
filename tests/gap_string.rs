#![feature(insert_str, plugin, type_ascription)]
#![plugin(quickcheck_macros)]

extern crate quickcheck;
extern crate viola;

use std::ops::Range;

use quickcheck::TestResult;
use viola::gap::GapString;

trait Splice {
    fn splice(&mut self, dest: Range<usize>, src: &str);
}

impl Splice for GapString {
    fn splice(&mut self, dest: Range<usize>, src: &str) {
        self.splice(dest, src);
    }
}

impl Splice for String {
    fn splice(&mut self, dest: Range<usize>, src: &str) {
        self.drain(dest.clone());
        self.insert_str(dest.start, src);
    }
}

fn valid_dest(dest: &Range<usize>, string: &str) -> bool {
    dest.start <= dest.end
        && string.is_char_boundary(dest.start)
        && string.is_char_boundary(dest.end)
}

#[quickcheck]
fn empty_one_splice(src: String) -> bool {
    let mut string = String::new();
    let mut buffer = GapString::new();

    string.splice(0..0, &src);
    buffer.splice(0..0, &src);

    string == buffer.into(): String
}

#[quickcheck]
fn one_splice(init: String, dest: Range<usize>, src: String) -> TestResult {
    if !valid_dest(&dest, &init) {
        return TestResult::discard();
    }

    let mut string = init.clone();
    let mut buffer = GapString::from(init);

    string.splice(dest.clone(), &src);
    buffer.splice(dest, &src);

    TestResult::from_bool(string == buffer.into(): String)
}

#[quickcheck]
fn multi_splice(init: String, splices: Vec<(Range<usize>, String)>) -> TestResult {
    let mut string = init.clone();
    let mut buffer = GapString::from(init);

    for (dest, src) in splices {
        if !valid_dest(&dest, &string) {
            return TestResult::discard();
        }

        string.splice(dest.clone(), &src);
        buffer.splice(dest, &src);
    }

    TestResult::from_bool(string == buffer.into(): String)
}
