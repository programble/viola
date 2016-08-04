//! Gap buffers.

pub mod buffer;
pub mod string;

/// Gap buffer iterator wrapper.
///
/// Wraps contiguous or fragmented iterators. Essentially a variation of `Chain`.
#[derive(Debug, Clone)]
pub struct GapIter<I: Iterator> {
    front: Option<I>,
    back: I,
    state: IterState,
}

#[derive(Debug, Clone, Copy)]
enum IterState {
    Both,
    Front,
    Back,
}

impl<I: Iterator> Iterator for GapIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        match self.state {
            IterState::Both => match self.front.as_mut().unwrap().next() {
                elt @ Some(..) => elt,
                None => {
                    self.state = IterState::Back;
                    self.back.next()
                },
            },
            IterState::Front => self.front.as_mut().unwrap().next(),
            IterState::Back => self.back.next(),
        }
    }

    // TODO: Implement other methods implemented by Chain.
}

impl<I: DoubleEndedIterator> DoubleEndedIterator for GapIter<I> {
    fn next_back(&mut self) -> Option<I::Item> {
        match self.state {
            IterState::Both => match self.back.next_back() {
                elt @ Some(..) => elt,
                None => {
                    self.state = IterState::Front;
                    self.front.as_mut().unwrap().next_back()
                },
            },
            IterState::Front => self.front.as_mut().unwrap().next_back(),
            IterState::Back => self.back.next_back(),
        }
    }
}
