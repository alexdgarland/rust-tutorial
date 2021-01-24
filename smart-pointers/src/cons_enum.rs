use std::fmt::{Display, Formatter, Result, Debug};
use List::Cons;
pub use List::Nil;

// TODO:
//  - drop_while
//  - iterative foreach?
//  - fold_right/ reduce_right?

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

}

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

    fn map<R, F: Fn(&T) -> R>(&self, f: F) -> List<R> {
        let add_new_value_to_list = |value: &T, mapped: List<R>| {
            cons(f(value), mapped)
        };
        let init: List<R> = Nil;
        self.reverse().fold_left(add_new_value_to_list, init)
    }

    fn reduce<F: Fn(&T, T) -> T>(&self, f: F) -> Option<T> {
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
            |value: &T, reversed: List<T>| {
                cons(value.clone(), reversed)
            },
            Nil
        )
    }

    fn filter<F: Fn(&T) -> bool>(&self, f: F) -> List<T> {
        let prepend_if_matches = |value: &T, filtered: List<T>| {
            if f(value) { cons(value.clone(), filtered) } else { filtered }
        };
        let prepended_list = self.fold_left(prepend_if_matches, Nil);
        prepended_list.reverse()
    }

    fn take(&self, n: usize) -> List<T> {
        fn inner<TT: Clone>(nn: usize, remaining: &List<TT>, processed: List<TT>) -> List<TT> {
            return match remaining {
                Cons(value, next, _) if nn > 0  => {
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
                Cons(value, next, _) if ff(value)  => {
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
                Cons(value, next, _) => {
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

    // TODO - big question here of whether we want to do one or both (as separate methods) of:
    //  - Return a reference to the sub-list, which nicely follows functional data-sharing behaviour
    //      and is very simple to implement in terms of the recursion,
    //      but due to Rust being non-garbage-collected will require significant care around lifetimes
    //  - Implement a copy/ clone of the sub-list - this makes more sense in terms of rust memory management (?)
    //      but makes the recursion more complex - could maybe make use of one of the following:
    //          - auto-derived clone behaviour (but would require a bound of List<T: Clone>
    //          - own implementation of either Clone or via (slightly hackily?) to-from vector methods??
    //          - completely fresh code?
    // fn drop_while<F: Fn(&T)-> bool>(&self, f: F) -> List<T> {
    //     fn inner<F: Fn(&T)-> bool, TT>(ff: FF, list: &List<TT>) -> List<TT> {
    //
    //     }
    //     inner(f, &self).clone()
    // }

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
