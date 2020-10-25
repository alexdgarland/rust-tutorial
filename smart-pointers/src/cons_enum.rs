use std::fmt::{Display, Formatter, Result};
use List::Cons;
pub use List::Nil;

// TODO - could add standard functional things like map, reduce, filter etc - maybe also an iterative foreach?

/// The implementation as defined in the exercise, using an enum
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let string = match self {
            Nil =>
                "".to_string(),
            Cons(value, boxed_list) =>
                {
                    let next = match &**boxed_list {
                        Nil =>
                            "".to_string(),
                        cons_list =>
                            format!(", {}", cons_list)
                    };
                    value.to_string() + &next
                }
        };
        write!(f, "{}", string)
    }
}

/// Function to make cons'ing slicker (take care of the required boxing)
pub fn cons<T>(value: T, list: List<T>) -> List<T> {
    Cons(value, Box::new(list))
}

pub fn demo_enum() {
    println!("***** Demoing enum implementation *****");
    let list = cons(1, cons(2, cons(3, Nil)));
    println!("{}", list)
}

#[cfg(test)]
mod tests {
    use super::{List, cons, Nil};
    use crate::test_helpers::WrappedInt;

    #[test]
    fn string_for_empty_list() {
        let nil: List<i32>= Nil;
        assert_eq!(nil.to_string(), "")
    }

    #[test]
    fn string_for_populated_list_i32() {
        let cons_list = cons(1, cons(2, cons(3, Nil)));
        assert_eq!(cons_list.to_string(), "1, 2, 3");
    }

    #[test]
    fn string_for_populated_list_str() {
        let cons_list = cons("one", cons("two", cons("three", Nil)));
        assert_eq!(cons_list.to_string(), "one, two, three");
    }

    #[test]
    fn string_for_populated_list_struct_with_display() {
        let cons_list =
            cons(WrappedInt { i: 1 },
                 cons(WrappedInt { i: 2 },
                      cons(WrappedInt { i: 3 }, Nil)));
        assert_eq!(cons_list.to_string(), "1, 2, 3");
    }

}
