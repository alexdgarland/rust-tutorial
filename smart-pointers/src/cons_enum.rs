use std::fmt::{Display, Formatter, Result};
use List::Cons;
pub use List::Nil;

// TODO - could also do an impl using a struct with an Option-al reference to another member,
//  rather than defining a separate value for Nil.
//  Could then make direct member access fully private and enforce using the cons function

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

// TODO - we actually want the to_string (Display) behaviour to work like:
//      - cons(1, cons(2, Nil)) -> "1, 2"
//      - Nil -> ""
impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Cons(value, list) = self {
            write!(f, "{}, {}", value, list.to_string())
        }
        else {
            write!(f, "NIL")
        }
    }
}

pub fn cons(value: i32, list: List) -> List {
    Cons(value, Box::new(list))
}

#[cfg(test)]
mod tests {

    use super::{cons, Nil};

    #[test]
    fn string_for_populated_list() {
        let cons_list = cons(1, cons(2, cons(3, Nil)));
        assert_eq!(cons_list.to_string(), "1, 2, 3, NIL")
    }

    #[test]
    fn string_for_empty_list() {
        assert_eq!(Nil.to_string(), "NIL")
    }

}