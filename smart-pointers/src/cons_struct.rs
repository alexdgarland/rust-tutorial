
// TODO - could make generic
// TODO - could add standard functional things like map, reduce, filter etc - maybe also an iterative foreach?

use std::fmt::{Display, Formatter, Result};

/// An alternative implementation, using Option rather than an Enum with a custom Nil type.
/// There are pros and cons to this approach - it allows usage of standard Option behaviour,
/// but does not allow us to customise behaviour of empty lists as they are just represented by None
/// and all operations on populated lists returned by cons have to work round the fact that it is wrapped in a Some.
pub struct List {
    value: i32,
    next: Option<Box<List>>
}

/// Function to make cons'ing slicker (take care of the required boxing)
fn cons(value: i32, list: Option<List>) -> Option<List> {
    Some(List {
        value, next:
        list.map(Box::new)
    })
}

impl Display for List {
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

pub fn demo_struct() {
    println!("***** Demoing struct implementation *****");
    let list = cons(1, cons(2, cons(3, None)));
    println!("{}", list.unwrap())
}

#[cfg(test)]
mod tests {
    use super::{cons, List};

    #[test]
    fn string_for_populated_list() {
        let cons_list: Option<List> = cons(1, cons(2, cons(3, None)));
        match cons_list {
            Some(list) =>
                assert_eq!(list.to_string(), "1, 2, 3"),
            _ =>
                assert!(false, "Function did not return expected type of Some<List>")
        }
    }

}
