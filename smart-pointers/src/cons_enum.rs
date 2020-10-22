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

impl Display for List {
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

pub fn cons(value: i32, list: List) -> List {
    Cons(value, Box::new(list))
}

#[cfg(test)]
mod tests {
    use super::{cons, Nil};

    #[test]
    fn string_for_populated_list() {
        let cons_list = cons(1, cons(2, cons(3, Nil)));
        assert_eq!(cons_list.to_string(), "1, 2, 3")
    }

    #[test]
    fn string_for_empty_list() {
        assert_eq!(Nil.to_string(), "")
    }
}
