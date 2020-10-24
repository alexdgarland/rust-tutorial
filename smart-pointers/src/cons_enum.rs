use std::fmt::{Display, Formatter, Result};
use List::Cons;
pub use List::Nil;

// TODO - could make generic
// TODO - could add standard functional things like map, reduce, filter etc - maybe also an iterative foreach?

/// The implementation as defined in the exercise, using an enum
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

/// Function to make cons'ing slicker (take care of the required boxing)
pub fn cons(value: i32, list: List) -> List {
    Cons(value, Box::new(list))
}

pub fn demo_enum() {
    println!("***** Demoing enum implementation *****");
    let list = cons(1, cons(2, cons(3, Nil)));
    println!("{}", list)
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
