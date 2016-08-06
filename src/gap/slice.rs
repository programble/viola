use range::{IntoRange, RangeExt};

/// Slice of a gap buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slice<'a> {
    /// Contiguous slice, i.e. entirely either side of the gap.
    Contiguous(&'a [u8]),

    /// Fragmented slice, i.e. separated by the gap.
    Fragmented(&'a [u8], &'a [u8]),
}

impl<'a> Slice<'a> {
    /// Returns the length of the slice.
    pub fn len(&self) -> usize {
        match *self {
            Slice::Contiguous(back) => back.len(),
            Slice::Fragmented(front, back) => front.len() + back.len(),
        }
    }

    /// Returns a sub-slice of the slice.
    ///
    /// # Panics
    ///
    /// Panics if the starting point is greater than the end point, or if either point is out of
    /// bounds.
    pub fn slice<R: IntoRange>(&self, range: R) -> Self {
        let range = range.into_range(self.len());
        match *self {
            Slice::Contiguous(back) => Slice::Contiguous(&back[range]),
            Slice::Fragmented(front, _) if range.end <= front.len() => {
                Slice::Contiguous(&front[range])
            },
            Slice::Fragmented(front, back) if range.start >= front.len() => {
                Slice::Contiguous(&back[range.sub(front.len())])
            },
            Slice::Fragmented(front, back) => {
                Slice::Fragmented(
                    &front[range.with_end(front.len())],
                    &back[..(range.end - front.len())],
                )
            },
        }
    }
}
