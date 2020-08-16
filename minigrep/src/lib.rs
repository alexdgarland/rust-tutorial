use std::error::Error;
use std::fs;

#[derive(Eq, PartialEq, Debug)]
pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Config<'a>, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config {
            query: &args[1],
            filename: &args[2],
        })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // A more declarative version than provided in the tutorial
    contents
        .lines()
        .filter(|l| l.contains(query))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Config, search};

    #[test]
    fn test_config_should_parse_correctly_from_valid_args() {
        let args = vec!["minigrep".to_string(), "some pattern".to_string(), "filename.type".to_string()];
        let expected = Config {
            query: "some pattern",
            filename: "filename.type"
        };
        assert_eq!(Config::new(&args), Ok(expected));
    }

    #[test]
    fn test_config_parsing_should_error_on_not_enough_args() {
        let args = vec!["minigrep".to_string(), "some pattern".to_string()];
        assert_eq!(Config::new(&args), Err("not enough arguments"));
    }


    #[test]
    fn test_search_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

}
