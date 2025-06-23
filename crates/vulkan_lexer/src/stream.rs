use std::str::Chars;

#[derive(Debug, Clone)]
pub struct CharStream<'input> {
    iter: Chars<'input>,
    offset: usize,
    prev: char,
    next: Option<char>,
}

const NULL: char = '\0';

impl<'input> CharStream<'input> {
    pub fn new(input: &'input str) -> Self {
        let mut iter = input.chars();
        let next = iter.next();
        Self {
            iter,
            offset: 0,
            prev: NULL,
            next,
        }
    }

    #[inline]
    pub fn next(&mut self) -> Option<char> {
        self.next.map(|c: char| {
            self.prev = c;
            self.next = self.iter.next();
            self.offset += c.len_utf8();
            c
        })
    }

    #[inline]
    pub fn peek(&self) -> Option<char> {
        self.next
    }
    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }

    #[inline]
    pub fn prev(&self) -> char {
        self.prev
    }
}
