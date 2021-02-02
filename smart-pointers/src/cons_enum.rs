use std::fmt::{Display, Formatter, Result, Debug};
use List::Cons;
pub use List::Nil;

/*

    NOTE - this is (I hope obviously!) a learning exercise, not a usable library.

    In particular, it is apparently not guaranteed that Rust will perform tail call optimisation reliably,
    hence use of tail-recursive functions is an intellectual exercise rather than a guarantee of good performance.

    Particularly unsure if recursive calls to struct-/ enum-attached like the following will see tail-call elimination.
    It seems they *could* but would need research/ experimentation to validate, which I probably won't take time to do!:

    fn recursive_method(&self, args) {
        if (base_case) {
            return some_non_recursive_value;
        }
        else {
            let new_args = do_something_to(args);
            // This seems like can pass all the info it needs through without holding a stack frame,
            // but not 100% clear  if reference to "self" will be properly  discarded by compiler
            // in favour of just working off of "next".
            return self.next.recursive_method(new_args);
        }
    }

 */


// TODO:
//  - fold_right/ reduce_right?
//  Following Scala conventions the signatures would be like:
// def foldLeft[B](z: B)(op: (B, A) => B): B
// Applies a binary operator to a start value and all elements of this list, going left to right.
//
// final def foldRight[B](z: B)(op: (A, B) => B): B
// Applies a binary operator to all elements of this list and a start value, going right to left.
//
// i.e. assumes that the way closure params are set up
// matches the direction we are moving through the list

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
            Cons(value, boxed_list, _size) =>
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

#[allow(dead_code)]
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
            if let Cons(value, next, _size) = &current_list
            {
                vector.push(value.clone());
                current_list = next;
            }
        }
        vector
    }

    fn for_each<F: FnMut(&T) -> ()>(&self, mut f: F) {
        if let Cons(value, next, _size) = self {
            f(value);
            next.for_each(f);
        }
    }

}

#[allow(dead_code)]
fn cons_list_from_vector<T: Clone>(vec: Vec<T>) -> List<T> {
    fn inner<TT: Clone>(remaining_slice: &[TT], processed_list: List<TT>) -> List<TT> {
        return match remaining_slice.split_last() {
            None =>
                processed_list,
            Some((last, init)) => {
                inner(init, cons(last.clone(), processed_list))
            }
        };
    }
    inner(&vec[..], Nil)
}

impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> Self {
        return self.map(|value| value.clone())
    }
}

#[allow(dead_code)]
impl<T: Clone> List<T> {

    fn fold_left<R, F: Fn(R, &T) -> R>(&self, init: R, f: F) -> R {
        match self {
            Nil =>
                init,
            Cons(value, next, _size) => {
                let result = f(init, value);
                next.fold_left(result, f)
            }
        }
    }

    fn map<R, F: Fn(&T) -> R>(&self, f: F) -> List<R> {
        let add_new_value_to_list = |mapped: List<R>, value: &T| {
            cons(f(value), mapped)
        };
        let init: List<R> = Nil;
        self.reverse().fold_left(init, add_new_value_to_list)
    }

    fn reduce_left<F: Fn(T, &T) -> T>(&self, f: F) -> Option<T> {
        match &self {
            Nil =>
                None,
            Cons(value, next, _size) => {
                Some(next.fold_left(value.clone(), f))
            }
        }
    }

    fn reverse(&self) -> List<T> {
        self.fold_left(
            Nil,
            |reversed: List<T>, value: &T| {
                cons(value.clone(), reversed)
            }
        )
    }

    fn filter<F: Fn(&T) -> bool>(&self, f: F) -> List<T> {
        let prepend_if_matches = |filtered: List<T>, value: &T| {
            if f(value) { cons(value.clone(), filtered) } else { filtered }
        };
        let prepended_list = self.fold_left(Nil, prepend_if_matches);
        prepended_list.reverse()
    }

    fn take(&self, n: usize) -> List<T> {
        fn inner<TT: Clone>(nn: usize, remaining: &List<TT>, processed: List<TT>) -> List<TT> {
            return match remaining {
                Cons(value, next, _size) if nn > 0  => {
                    let next_list = cons(value.clone(), processed);
                    inner(nn - 1, next, next_list)
                }
                _ =>
                    processed
            }
        }
        return inner(n, &self, Nil).reverse()
    }

    fn take_while<F: Fn(&T)-> bool>(&self, f: F) -> List<T> {
        fn inner<TT: Clone, FF: Fn(&TT) -> bool>(
            ff: FF, remaining: &List<TT>, processed: List<TT>
        ) -> List<TT> {
            return match remaining {
                Cons(value, next, _size) if ff(value)  => {
                    let next_list = cons(value.clone(), processed);
                    inner(ff, next, next_list)
                }
                _ =>
                    processed
            }
        }
        return inner(f, &self, Nil).reverse()
    }

    fn drop(&self, n: usize) -> List<T> {
        fn inner<TT: Clone>(nn: usize, list: &List<TT>) -> List<TT> {
            match list {
                Nil =>
                    Nil,
                Cons(_value, next, _size) => {
                    if nn <= 1 {
                        (next as &List<TT>).clone()
                    }
                    else {
                        inner(nn -1, next)
                    }
                }
            }
        }
        return inner(n, self)
    }

    fn drop_while<F: Fn(&T)-> bool>(&self, f: F) -> List<T> {
        fn inner<FF: Fn(&TT)-> bool, TT>(ff: FF, list: &List<TT>) -> &List<TT> {
            match list {
                Cons(value, next, _size) => {
                    if ff(value) { inner(ff, next) }
                    else { list }
                },
                Nil =>
                    &Nil
            }
        }
        inner(f, &self).clone()
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
mod tests;
