use std::collections::HashMap;
use std::hash::Hash;

pub struct Cacher<A, R, F>
    where
        A: Eq + Hash,
        R: Clone,
        F: FnMut(&A) -> R
{
    calculation: F,
    map: HashMap<A, R>,
}

impl<A, R, F> Cacher<A, R, F>
    where
        A: Eq + Hash,
        R: Clone,
        F: FnMut(&A) -> R
{
    pub fn new(calculation: F) -> Cacher<A, R, F> {
        Cacher {
            calculation,
            map: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: A) -> R {
        match self.map.get(&arg) {
            Some(v) =>
                (*v).clone(),
            None => {
                let v = (self.calculation)(&arg);
                self.map.insert(arg, v.clone());
                v.clone()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cacher::Cacher;

    #[test]
    fn test_cacher_int_int() {
        let mut calls: Vec<u32> = Vec::new();

        let mut cacher  = Cacher::new(
            |arg: &u32| {
                calls.push(arg.clone());
                arg * 2
            }
        );

        assert_eq!(cacher.value(2), 4, "Initial call with value 2 should return expected value");
        assert_eq!(cacher.value(3), 6, "Call with value 3 should return expected value");
        assert_eq!(cacher.value(2), 4, "Repeat call with value 2 should return expected value");
        assert_eq!(calls, vec![2, 3], "There should only be one call made for each distinct value");
    }

    #[test]
    fn test_cacher_string_string() {
        let mut calls: Vec<String> = Vec::new();

        let mut cacher  = Cacher::new(
            |arg: &String| {
                calls.push(arg.clone());
                format!("Hello {}!", arg)
            });

        assert_eq!(cacher.value("world".to_string()), "Hello world!",
                   "Initial call with value \"world\" should return expected value");
        assert_eq!(cacher.value("Bob".to_string()), "Hello Bob!",
                   "Call with value \"Bob\" should return expected value");
        assert_eq!(cacher.value("world".to_string()), "Hello world!",
                   "Repeat call with value \"world\" should return expected value");
        assert_eq!(calls, vec!["world", "Bob"], "There should only be one call made for each distinct value")
    }

}
