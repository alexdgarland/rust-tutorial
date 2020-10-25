
// TODO - could add standard functional things like map, reduce, filter etc - maybe also an iterative foreach?

use std::fmt::{Display, Formatter, Result};

/// An alternative implementation, using Option rather than an Enum with a custom Nil type.
/// There are pros and cons to this approach - it allows usage of standard Option behaviour,
/// but does not allow us to customise behaviour of empty lists as they are just represented by None
/// and all operations on populated lists returned by cons have to work round the fact that it is wrapped in a Some.
pub struct List<T> {
    value: T,
    next: Option<Box<List<T>>>
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let next = match &self.next {
            None =>
                "".to_string(),
            Some(list) =>
                format!(", {}", list)
        };
        write!(f, "{}{}", &self.value, next)
    }
}

/// Function to make cons'ing slicker (take care of the required boxing)
fn cons<T>(value: T, list: Option<List<T>>) -> Option<List<T>> {
    Some(List { value, next: list.map(Box::new) })
}

pub fn demo_struct() {
    println!("***** Demoing struct implementation *****");
    let list = cons(1, cons(2, cons(3, None)));
    println!("{}", list.unwrap())
}

#[cfg(test)]
mod tests {
    use super::{cons, List};
    use std::fmt::Display;
    use crate::test_helpers::WrappedInt;

    fn assert_some_tostring<T: Display>(cons_list: Option<List<T>>, expected_string: &str) {
        match cons_list {
            Some(list) =>
                assert_eq!(list.to_string(), expected_string),
            _ =>
                assert!(false, "Function did not return expected type of Some<List>")
        }
    }

    #[test]
    fn string_for_populated_list_i32() {
        let cons_list: Option<List<i32>> = cons(1, cons(2, cons(3, None)));
        assert_some_tostring(cons_list, "1, 2, 3");
    }

    #[test]
    fn string_for_populated_list_str() {
        let cons_list: Option<List<&str>> = cons("one", cons("two", cons("three", None)));
        assert_some_tostring(cons_list,"one, two, three");
    }

    #[test]
    fn string_for_populated_list_struct_with_display() {
        let cons_list =
            cons(WrappedInt { i: 1 },
                 cons(WrappedInt { i: 2 },
                      cons(WrappedInt { i: 3 }, None)));
        assert_some_tostring(cons_list, "1, 2, 3");
    }

}
