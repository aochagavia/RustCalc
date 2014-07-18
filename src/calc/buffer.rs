/*

Implements a buffer to be used when reading chars or tokens

*/

pub struct Buffer<T, U> {
    stack: Vec<T>,
    iterator: U
}

impl<T: Copy, U: Iterator<T>> Buffer<T, U> {
    pub fn new(chars: U) -> Buffer<T, U> {
        Buffer { stack: vec![], iterator: chars }
    }

    pub fn is_empty(&mut self) -> bool {
        match self.pop() {
            Some(c) => {
                self.push(c);
                false
            }
            None    => true
        }
    }

    pub fn take_until(&mut self, f: |&T| -> bool) -> Vec<T> {
        let mut vec = vec![];
        loop {
            match self.pop() {
                Some(x) => {
                    if f(&x) { self.push(x); return vec; }
                    else    { vec.push(x); }
                }
                None    => { return vec }
            }
        }
    }

    pub fn peek(&mut self) -> Option<T> {
        match self.stack.pop() {
            Some(c) => {
                self.push(c);
                Some(c)
            }
            None    => None
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop().or_else(|| self.iterator.next())
    }

    pub fn push(&mut self, elem: T) {
        self.stack.push(elem);
    }
}
