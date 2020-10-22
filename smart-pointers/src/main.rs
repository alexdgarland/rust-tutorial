
use smart_pointers::cons_enum::{cons, Nil};

fn main() {
    let list = cons(1, cons(2, cons(3, Nil)));
    print!("{}\n", list)
}
