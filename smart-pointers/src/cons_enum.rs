use std::fmt::{Display, Formatter, Result, Debug};
use List::Cons;
pub use List::Nil;

// TODO:
//  - take
//  - take_while
//  - drop
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
            |value: &T, reversed: List<T>| {
                cons(value.clone(), reversed)
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
