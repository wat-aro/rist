use List::Cons;

#[derive(Debug, PartialEq)]
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

pub fn cons<T>(x: T, xs: List<T>) -> List<T> {
    Cons(x, Box::new(xs))
}

#[cfg(test)]
mod tests {
    use super::List::{Cons, Nil};
    use super::*;

    #[test]
    fn cons_x_list() {
        let list = cons(1, cons(2, Nil));

        assert_eq!(list, Cons(1, Box::new(Cons(2, Box::new(Nil)))));
    }
}
