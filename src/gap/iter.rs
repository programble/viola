use std::slice::Iter as SliceIter;
use std::str::Chars;

use super::{Slice, Str};

/// Gap buffer iterator wrapper.
///
/// Wraps contiguous or fragmented iterators. Essentially a variation of `Chain`.
#[derive(Debug, Clone)]
pub struct Iter<I: Iterator> {
    front: Option<I>,
    back: I,
    state: State,
}

#[derive(Debug, Clone, Copy)]
enum State {
    Both,
    Front,
    Back,
}

impl<I: Iterator> Iterator for Iter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        match self.state {
            State::Both => match self.front.as_mut().unwrap().next() {
                elt @ Some(..) => elt,
                None => {
                    self.state = State::Back;
                    self.back.next()
                },
            },
            State::Front => self.front.as_mut().unwrap().next(),
            State::Back => self.back.next(),
        }
    }

    // TODO: Implement other methods implemented by Chain?
}

impl<I: DoubleEndedIterator> DoubleEndedIterator for Iter<I> {
    fn next_back(&mut self) -> Option<I::Item> {
        match self.state {
            State::Both => match self.back.next_back() {
                elt @ Some(..) => elt,
                None => {
                    self.state = State::Front;
                    self.front.as_mut().unwrap().next_back()
                },
            },
            State::Front => self.front.as_mut().unwrap().next_back(),
            State::Back => self.back.next_back(),
        }
    }
}

impl<'a> IntoIterator for Slice<'a> {
    type Item = &'a u8;
    type IntoIter = Iter<SliceIter<'a, u8>>;

    fn into_iter(self) -> Iter<SliceIter<'a, u8>> {
        match self {
            Slice::Contiguous(back) => Iter {
                front: None,
                back: back.into_iter(),
                state: State::Back,
            },
            Slice::Fragmented(front, back) => Iter {
                front: Some(front.into_iter()),
                back: back.into_iter(),
                state: State::Both,
            },
        }
    }
}

impl<'a> Str<'a> {
    /// Returns an iterator over the chars of the string slice.
    pub fn chars(&self) -> Iter<Chars<'a>> {
        match *self {
            Str::Contiguous(back) => Iter {
                front: None,
                back: back.chars(),
                state: State::Back,
            },
            Str::Fragmented(front, back) => Iter {
                front: Some(front.chars()),
                back: back.chars(),
                state: State::Both,
            },
        }
    }
}
