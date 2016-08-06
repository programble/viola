use std::fmt::{Debug, Display, Error, Formatter};

use super::{Buffer, Slice, Str, String};

struct Gap(usize);

impl Debug for Gap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[..{}..]", self.0)
    }
}

impl Debug for Buffer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let gap = Gap(self.gap.len());
        match self.as_slice() {
            Slice::Contiguous(back) if self.gap.start == 0 => {
                f.debug_list().entry(&gap).entries(back).finish()
            },
            Slice::Contiguous(front) => {
                f.debug_list().entries(front).entry(&gap).finish()
            },
            Slice::Fragmented(front, back) => {
                f.debug_list().entries(front).entry(&gap).entries(back).finish()
            },
        }
    }
}

impl Debug for String {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let gap = Gap(self.buf.gap.len());
        match self.as_str() {
            Str::Contiguous(back) if self.buf.gap.start == 0 => {
                f.debug_list().entry(&gap).entry(&back).finish()
            },
            Str::Contiguous(front) => {
                f.debug_list().entry(&front).entry(&gap).finish()
            },
            Str::Fragmented(front, back) => {
                f.debug_list().entry(&front).entry(&gap).entry(&back).finish()
            },
        }
    }
}

impl Display for String {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> Display for Str<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Str::Contiguous(back) => f.write_str(back),
            Str::Fragmented(front, back) => {
                f.write_str(front)?;
                f.write_str(back)
            },
        }
    }
}
