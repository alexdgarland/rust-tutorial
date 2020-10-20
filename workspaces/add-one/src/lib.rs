use rand;

pub fn add_one(x: i32) -> i32 {
    println!("Adding one (not adding a random number such as {}!)", rand::random::<u8>());
    x+ 1
}

#[cfg(test)]
mod tests {
    use super::add_one;

    #[test]
    fn adds_one_to_two() {
        assert_eq!(add_one(2), 3);
    }
}
