use std::fmt::{Display, Formatter, Result, Debug};
use List::Cons;
pub use List::Nil;

// TODO - could add standard functional things like map, reduce, filter etc - maybe also an iterative foreach?

/// Based on the implementation as defined in the exercise, using an enum
#[derive(PartialEq)]
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

impl<T: Display> Debug for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "List with size {} (elements [{}])", &self.length(), &self)
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
            Nil =>
                Nil,
            Cons(value, next, _) =>
                cons(f(value), next.map(f))
        }

    }

}

impl<T: Clone> List<T> {

    fn fold_left<R, F: Fn(&T, R) -> R>(&self, f: F, init: R) -> R {
        match self {
            Nil =>
                init,
            Cons(value, next, _) => {
                let result = f(value, init);
                next.fold_left(f, result)
            }
        }
    }

    fn reduce(&self, f: fn(&T, T) -> T) -> Option<T> {
        match &self {
            Nil =>
                None,
            Cons(value, next, _) => {
                Some(next.fold_left(f, value.clone()))
            }
        }
    }

    fn reverse(&self) -> List<T> {
        self.fold_left(
            |value: &T, processed_list: List<T>| {
                cons(value.clone(), processed_list)
            },
            Nil
        )
    }

    fn filter(&self, f: fn(&T) -> bool) -> List<T> {
        let prepend_if_matches = |value: &T, list: List<T>| {
            if f(value) { cons(value.clone(), list) } else { list }
        };
        let prepended_list = self.fold_left(prepend_if_matches, Nil);
        prepended_list.reverse()
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

    fn nil_int_list() -> List<i32> {
        Nil
    }

    fn example_int_list() -> List<i32> {
        cons(1, cons(2, cons(3, Nil)))
    }

    #[test]
    fn length_for_empty_list() {
        assert_eq!(nil_int_list().length(), 0)
    }

    #[test]
    fn length_for_populated_list_i32() {
        assert_eq!(example_int_list().length(), 3);
    }

    #[test]
    fn string_for_empty_list() {
        assert_eq!(nil_int_list().to_string(), "")
    }

    #[test]
    fn string_for_populated_list_i32() {
        assert_eq!(
            example_int_list().to_string(),
            "1, 2, 3"
        );
    }

    #[test]
    fn string_for_populated_list_str() {
        let cons_list =
            cons("one",
                 cons("two",
                      cons("three", Nil)));
        assert_eq!(
            cons_list.to_string(),
            "one, two, three"
        );
    }

    #[test]
    fn string_for_populated_list_struct_with_display() {
        let cons_list =
            cons(WrappedInt { i: 1 },
                 cons(WrappedInt { i: 2 },
                      cons(WrappedInt { i: 3 }, Nil)));
        assert_eq!(
            cons_list.to_string(),
            "1, 2, 3"
        );
    }

    #[test]
    fn to_vector_for_empty_list() {
        let expected: Vec<&i32> = vec![];
        assert_eq!(
            nil_int_list().to_vector(),
            expected
        );
    }

    #[test]
    fn to_vector_for_populated_list_i32() {
        assert_eq!(
            example_int_list().to_vector(),
            vec![&1, &2, &3]
        );
    }

    #[test]
    fn map_for_empty_list() {
        assert_eq!(
            nil_int_list().map(|i:&i32| i + 1).to_string(),
            ""
        );
    }

    #[test]
    fn map_for_populated_list_i32() {
        assert_eq!(
            example_int_list().map(|i:&i32| i + 1).to_vector(),
            vec!(&2, &3, &4)
        );
    }

    fn add(i: &i32, j: i32) -> i32 {
        return i + j
    }

    #[test]
    fn reduce_for_empty_list() {
        assert_eq!(
            nil_int_list().reduce(add),
            None
        );
    }

    #[test]
    fn reduce_for_populated_list_i32() {
        assert_eq!(
            example_int_list().reduce(add),
            Some(6)
        );
    }

    fn join_strings(i: &i32, s: String) -> String {
        return format!("{}, {}", s, i);
    }

    #[test]
    fn fold_for_empty_list() {
        assert_eq!(
            nil_int_list().fold_left(join_strings, "0".to_owned()),
            "0"
        );
    }

    #[test]
    fn fold_for_populated_list_i32() {
        assert_eq!(
            example_int_list().fold_left(join_strings, "0".to_owned()),
            "0, 1, 2, 3"
        );
    }

    fn is_even(i: &i32) -> bool {
        return i % 2 == 0
    }

    #[test]
    fn filter_for_empty_list() {
        assert_eq!(
            nil_int_list().filter(is_even),
            Nil
        );
    }

    #[test]
    fn filter_for_populated_list_i32() {
        assert_eq!(
            example_int_list().filter(is_even),
            cons(2, Nil)
        );
    }

    #[test]
    fn reverse_for_empty_list() {
        assert_eq!(
            nil_int_list().reverse(),
            Nil
        );
    }

    #[test]
    fn reverse_for_populated_list_i32() {
        assert_eq!(
            example_int_list().reverse(),
            cons(3, cons(2, cons(1, Nil)))
        );
    }

}
