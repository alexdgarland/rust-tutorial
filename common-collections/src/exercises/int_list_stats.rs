use std::collections::HashMap;

use std::fmt::{Display, Formatter, Result};

pub struct IntList {
    pub list: Vec<u32>
}

impl IntList {
    fn apply_to_nonempty_list<T>(&self, f: fn(&Vec<u32>) -> T) -> Option<T> {
        match &self.list {
            l if l.is_empty() => None,
            populated_list => Some(f(populated_list))
        }
    }

    fn mean(&self) -> Option<f32> {
        self.apply_to_nonempty_list(|list| {
            list.iter().map(|i| *i as f32).sum::<f32>() / list.len() as f32
        })
    }

    fn median(&self) -> Option<u32> {
        self.apply_to_nonempty_list(|list| {
            let mut sorted = list.clone();
            sorted.sort();
            sorted[sorted.len() / 2]
        })
    }

    fn mode(&self) -> Option<u32> {
        let mut map: HashMap<u32, u32> = HashMap::new();
        let mut current_max_count: u32 = 0;
        let mut current_mode_candidate: Option<u32> = None;

        for item_value in self.list.iter() {
            let count = map.entry(*item_value).or_insert(0);
            *count += 1;
            if count > &mut current_max_count {
                current_max_count = *count;
                current_mode_candidate = Some(*item_value);
            }
        }

        return current_mode_candidate;
    }
}

impl Display for IntList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        fn show_optional<T: Display>(optional: Option<T>) -> String {
            match optional {
                None => "(N/A - empty list)".to_string(),
                Some(value) => value.to_string()
            }
        }

        write!(
            f, "IntList: {:?}\nMean: {}, median: {}, mode: {}\n",
            self.list,
            show_optional(self.mean()),
            show_optional(self.median()),
            show_optional(self.mode())
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::exercises::int_list_stats::IntList;

    #[test]
    fn test_mean() {
        let list = IntList {
            list: vec![1, 1, 2, 2, 4, 4, 5, 5]
        };
        assert_eq!(list.mean(), Some(3.0));
    }

    #[test]
    fn test_mean_empty_list() {
        let list = IntList {
            list: vec![]
        };
        assert_eq!(list.mean(), None);
    }

    #[test]
    fn test_median() {
        let list = IntList {
            list: vec![7, 6, 9, 3, 3, 7, 1]
        };
        assert_eq!(list.median(), Some(6));
    }

    #[test]
    fn test_median_empty_list() {
        let list = IntList {
            list: vec![]
        };
        assert_eq!(list.median(), None);
    }

    #[test]
    fn test_mode() {
        let list = IntList {
            list: vec![1, 2, 3, 4, 6, 6, 7, 7, 7, 8, 9]
        };
        assert_eq!(list.mode(), Some(7));
    }

    #[test]
    fn test_mode_empty_list() {
        let list = IntList {
            list: vec![]
        };
        assert_eq!(list.mode(), None);
    }
}
