use std::fmt::{Debug, Display, Formatter, Error};

use super::{GapBuffer, GapString};

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
            .entry(&Gap(self.gap.len()))
            .entries(b)
            .finish()
    }
}

impl Debug for GapString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let (a, b) = self.as_strs();
        f.debug_tuple("")
            .field(&a)
            .field(&Gap(self.buf.gap.len()))
            .field(&b)
            .finish()
    }
}

impl Display for GapString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let (a, b) = self.as_strs();
        f.write_str(a)?;
        f.write_str(b)
    }
}
