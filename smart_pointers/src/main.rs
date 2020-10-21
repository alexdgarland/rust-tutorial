use List::{Cons, Nil};
use std::fmt::{Display, Formatter};
use std::fmt;

// TODO - the List enum here can be moved out into a proper lib file and tested
//  e.g. we actually want the to_string (Display) behaviour to handle things like:
//      - cons(1, cons(2, Nil)) -> "1, 2"
//      - Nil -> ""

enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Cons(value, list) = self {
            write!(f, "{}, {}", value, list.to_string())
        }
        else {
            write!(f, "NIL")
        }
    }
}

fn cons(value: i32, list: List) -> List {
    Cons(value, Box::new(list))
}

// TODO - could also do an impl using a struct with an Option-al reference to another member,
//  rather than defining a separate value for Nil.
//  Could then make direct member access private and enforce using the cons function

fn main() {
    let list = cons(1, cons(2, cons(3, Nil)));
    print!("{}", list)
}
