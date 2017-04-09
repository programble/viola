//! Splice operations.

use std::ops::Range;
use std::string::String as StdString;

use gap::{Buffer, String};

/// Splice operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operation<T> {
    /// Destination range.
    pub dest: Range<usize>,

    /// Source data.
    pub src: T,
}

/// Types which can have splice operations applied.
pub trait Operate {
    /// Simple owned data container.
    type Owned;

    /// Applies a splice operation, returning the corresponding revert operation.
    fn apply(&mut self, operation: &Operation<Self::Owned>) -> Operation<Self::Owned>;
}

impl Operate for Buffer {
    type Owned = Vec<u8>;

    fn apply(&mut self, operation: &Operation<Vec<u8>>) -> Operation<Vec<u8>> {
        let src = self.slice(operation.dest.clone()).into();
        let dest = self.splice(operation.dest.clone(), &operation.src);
        Operation {
            dest: dest,
            src: src,
        }
    }
}

impl Operate for String {
    type Owned = StdString;

    fn apply(&mut self, operation: &Operation<StdString>) -> Operation<StdString> {
        let src = self.slice(operation.dest.clone()).into();
        let dest = self.splice(operation.dest.clone(), &operation.src);
        Operation {
            dest: dest,
            src: src,
        }
    }
}
