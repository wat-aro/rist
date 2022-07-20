//! # List
//!
//! `rist` is a utilities for List.

use List::{Cons, Nil};

#[derive(Debug, PartialEq)]
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T> List<T> {
    /// Construct List
    ///
    /// # Examples
    ///
    ///
    /// ```
    /// use rist::List;
    /// use rist::List::{Cons, Nil};
    ///
    /// let list = List::cons(1, List::cons(2, Nil));
    ///
    /// assert_eq!(list, Cons(1, Box::new(Cons(2, Box::new(Nil)))));
    /// ```
    pub fn cons(x: T, xs: List<T>) -> List<T> {
        Cons(x, Box::new(xs))
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { list: self }
    }
}

pub struct Iter<'a, T> {
    pub list: &'a List<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list {
            Nil => None,
            Cons(x, xs) => {
                self.list = xs;
                Some(x)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List::{Cons, Nil};
    use super::*;

    #[test]
    fn cons_x_list() {
        let list = List::cons(1, List::cons(2, Nil));

        assert_eq!(list, Cons(1, Box::new(Cons(2, Box::new(Nil)))));
    }

    #[test]
    fn next_list() {
        let list = List::cons(1, List::cons(2, Nil));
        let mut list_iter = list.iter();

        assert_eq!(list_iter.next(), Some(&1));
        assert_eq!(list_iter.next(), Some(&2));
        assert_eq!(list_iter.next(), None);
        assert_eq!(list_iter.next(), None);
    }
}
