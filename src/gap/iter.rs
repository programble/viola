use std::slice::Iter as SliceIter;
use std::str::{Chars, CharIndices as StrCharIndices};

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

impl<I: Iterator> Iter<I> {
    fn front(&mut self) -> &mut I {
        self.front.as_mut().expect("missing front iterator")
    }
}

impl<I: Iterator> Iterator for Iter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        match self.state {
            State::Both => match self.front().next() {
                elt @ Some(..) => elt,
                None => {
                    self.state = State::Back;
                    self.back.next()
                },
            },
            State::Front => self.front().next(),
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
                    self.front().next_back()
                },
            },
            State::Front => self.front().next_back(),
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

    /// Returns an iterator over the chars of a slice, and their positions.
    pub fn char_indices(&self) -> CharIndices<'a> {
        match *self {
            Str::Contiguous(back) => CharIndices {
                front_len: 0,
                front: None,
                back: back.char_indices(),
                state: State::Back,
            },
            Str::Fragmented(front, back) => CharIndices {
                front_len: front.len(),
                front: Some(front.char_indices()),
                back: back.char_indices(),
                state: State::Both,
            },
        }
    }
}

/// Iterator for a gap buffer string's characters and their byte offsets.
#[derive(Debug, Clone)]
pub struct CharIndices<'a> {
    front_len: usize,
    front: Option<StrCharIndices<'a>>,
    back: StrCharIndices<'a>,
    state: State,
}

impl<'a> CharIndices<'a> {
    fn front(&mut self) -> &mut StrCharIndices<'a> {
        self.front.as_mut().expect("missing front iterator")
    }

    fn map_back(&self, item: (usize, char)) -> (usize, char) {
        let (index, ch) = item;
        (index + self.front_len, ch)
    }
}

impl<'a> Iterator for CharIndices<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<(usize, char)> {
        match self.state {
            State::Both => match self.front().next() {
                elt @ Some(..) => elt,
                None => {
                    self.state = State::Back;
                    self.back.next().map(|x| self.map_back(x))
                },
            },
            State::Front => self.front().next(),
            State::Back => self.back.next().map(|x| self.map_back(x)),
        }
    }
}

impl<'a> DoubleEndedIterator for CharIndices<'a> {
    fn next_back(&mut self) -> Option<(usize, char)> {
        match self.state {
            State::Both => match self.back.next_back() {
                Some(item) => Some(self.map_back(item)),
                None => {
                    self.state = State::Front;
                    self.front().next_back()
                },
            },
            State::Front => self.front().next_back(),
            State::Back => self.back.next_back().map(|x| self.map_back(x)),
        }
    }
}
