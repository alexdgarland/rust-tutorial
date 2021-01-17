use std::fmt::{Display, Formatter, Result};
use List::Cons;
pub use List::Nil;

// TODO - could add standard functional things like map, reduce, filter etc - maybe also an iterative foreach?

/// Based on the implementation as defined in the exercise, using an enum
pub enum List<T> {
    Cons(T, Box<List<T>>, usize),
    Nil,
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let string = match self {
            Nil =>
                "".to_string(),
            Cons(value, boxed_list, _) =>
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

impl<T> List<T> {

    fn length(&self) -> usize {
        match &self {
            Nil => 0,
            Cons(_, _, size) => *size
        }
    }

    /// Convert to vector - to be efficient, this is inherently a mutating operation,
    /// so not trying to do in a fully functional/ immutable way.
    fn to_vector(&self) -> Vec<&T> {
        let length = *&self.length();
        let mut vector: Vec<&T> = Vec::with_capacity(length);
        let mut current_list: &List<T> = &self;
        for _ in 0..length {
            if let Cons(value, next, _) = &current_list
            {
                vector.push(value.clone());
                current_list = next;
            }
        }
        vector
    }

    fn map<R>(&self, f: fn(&T) -> R) -> List<R> {
        match &self {
            Nil => Nil,
            Cons(value, next, _) => {
                cons(f(value), next.map(f))
            }
        }

    }

}

/// Function to make cons'ing slicker (take care of the required boxing)
pub fn cons<T>(value: T, list: List<T>) -> List<T> {
    let new_length = list.length() + 1;
    Cons(value, Box::new(list), new_length)
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
    fn length_for_empty_list() {
        let nil: List<i32>= Nil;
        assert_eq!(nil.length(), 0)
    }

    #[test]
    fn length_for_populated_list_i32() {
        let cons_list = cons(1, cons(2, cons(3, Nil)));
        assert_eq!(cons_list.length(), 3);
    }

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

    #[test]
    fn to_vector_for_empty_list() {
        let nil: List<i32>= Nil;
        let expected: Vec<&i32> = vec![];
        assert_eq!(nil.to_vector(), expected);
    }

    #[test]
    fn to_vector_for_populated_list_i32() {
        let cons_list = cons(1, cons(2, cons(3, Nil)));
        assert_eq!(cons_list.to_vector(), vec![&1, &2, &3]);
    }

    #[test]
    fn map_for_empty_list() {
        let nil: List<i32>= Nil;
        assert_eq!(nil.map(|i:&i32| i + 1).to_string(), "");
    }

    #[test]
    fn map_for_populated_list_i32() {
        let cons_list = cons(1, cons(2, cons(3, Nil)));
        assert_eq!(cons_list.map(|i:&i32| i + 1).to_vector(), vec!(&2, &3, &4));
    }

}
