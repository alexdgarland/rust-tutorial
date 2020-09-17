
pub(crate) fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // A more declarative version than provided in the tutorial
    contents
        .lines()
        .filter(|l| l.contains(query))
        .collect()
}

pub(crate) fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lowercase = query.to_lowercase();
    contents
        .lines()
        .filter(|l| l.to_lowercase().contains(&query_lowercase))
        .collect()
}

#[cfg(test)]
mod tests {

    use super::{search, search_case_insensitive};

    #[test]
    fn test_search_case_sensitive_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_search_case_insensitive_one_result() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

}