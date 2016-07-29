use std::fmt::{Debug, Formatter, Error};

use super::buffer::GapBuffer;

struct Gap(usize);

impl Debug for Gap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[..{}..]", self.0)
    }
}

impl Debug for GapBuffer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let (a, b) = self.as_slices();
        f.debug_list()
            .entries(a)
            .entry(&Gap(self.gap_len()))
            .entries(b)
            .finish()
    }
}
