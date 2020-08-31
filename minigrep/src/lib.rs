use std::error::Error;
use std::fs;
use std::env;
use mockall_derive::automock;

type EnvVarResult = Result<String, env::VarError>;

#[automock]
trait VariableAccess {
    fn get_var(&self, var: &'static str) -> EnvVarResult;
}

struct EnvironmentVariableAccessor;

impl VariableAccess for EnvironmentVariableAccessor {
    fn get_var(&self, var: &'static str) -> EnvVarResult {
        env::var(var)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool
}

impl<'a> Config<'a> {

    fn new_specifying_var_source<V: VariableAccess>(args: &'a [String], env_var_source: V) -> Result<Config<'a>, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config {
            query: &args[1],
            filename: &args[2],
            case_sensitive: env_var_source.get_var("CASE_INSENSITIVE").is_err()
        })
    }

    pub fn new(args: &'a [String]) -> Result<Config<'a>, &str> {
        Config::new_specifying_var_source(args, EnvironmentVariableAccessor{})
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // A more declarative version than provided in the tutorial
    contents
        .lines()
        .filter(|l| l.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lowercase = query.to_lowercase();
    contents
        .lines()
        .filter(|l| l.to_lowercase().contains(&query_lowercase))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Config, MockVariableAccess, search, search_case_insensitive, EnvVarResult};
    use mockall::predicate::eq;
    use std::env::VarError;

    fn run_test_parse_valid_args(case_insensitive_env_arg_return: EnvVarResult, expected_case_sensitive: bool) {
        let args = vec!["minigrep".to_string(), "some pattern".to_string(), "filename.type".to_string()];

        let mut mock_var_access = MockVariableAccess::new();
        mock_var_access
            .expect_get_var()
            .times(1)
            .with(eq("CASE_INSENSITIVE"))
            .return_const(case_insensitive_env_arg_return);

        let expected = Config {
            query: "some pattern",
            filename: "filename.type",
            case_sensitive: expected_case_sensitive
        };

        assert_eq!(
            Config::new_specifying_var_source(&args, mock_var_access),
            Ok(expected)
        );
    }

    #[test]
    fn test_config_should_parse_correctly_from_valid_args_case_sensitive_true() {
        run_test_parse_valid_args(Ok("SOME_SET_VALUE".to_string()), false);
    }

    #[test]
    fn test_config_should_parse_correctly_from_valid_args_case_sensitive_false() {
        run_test_parse_valid_args(Err(VarError::NotPresent), true);
    }

    #[test]
    fn test_config_parsing_should_error_on_not_enough_args() {
        let args = vec!["minigrep".to_string(), "some pattern".to_string()];

        assert_eq!(
            Config::new_specifying_var_source(&args, MockVariableAccess::new()),
            Err("not enough arguments")
        );
    }


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
