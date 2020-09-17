use std::env;
use mockall_derive::automock;

type EnvVarResult = Result<String, env::VarError>;

#[automock]
trait VariableAccess {
    fn get_var(&self, key: &'static str) -> EnvVarResult;
}

struct EnvironmentVariableAccessor;

impl VariableAccess for EnvironmentVariableAccessor {
    fn get_var(&self, key: &'static str) -> EnvVarResult {
        env::var(key)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {

    fn new_specifying_var_source<I: Iterator<Item = String>, V: VariableAccess>(mut args: I, env_var_source: V)
                                                                                 -> Result<Config, &'static str>
    {
        let _program_name = args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };

        let case_sensitive = env_var_source.get_var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }

    pub fn new<I: Iterator<Item = String>>(args: I) -> Result<Config, &'static str> {
        Config::new_specifying_var_source(args, EnvironmentVariableAccessor{})
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, MockVariableAccess, EnvVarResult};
    use mockall::predicate::eq;
    use std::env::VarError;

    fn run_test_parse_valid_args(case_insensitive_env_arg_return: EnvVarResult, expected_case_sensitive: bool) {
        
        let mut args = vec!["minigrep".to_string(), "some query".to_string(), "filename.type".to_string()];

        let mut mock_var_access = MockVariableAccess::new();
        mock_var_access
            .expect_get_var()
            .times(1)
            .with(eq("CASE_INSENSITIVE"))
            .return_const(case_insensitive_env_arg_return);

        let expected = Config {
            query: "some query".to_string(),
            filename: "filename.type".to_string(),
            case_sensitive: expected_case_sensitive
        };

        assert_eq!(
            Config::new_specifying_var_source(args.drain(..), mock_var_access),
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

    fn run_test_missing_arg(mut args: Vec<String>, expected_error_message: &'static str) {
        assert_eq!(
            Config::new_specifying_var_source(args.drain(..), MockVariableAccess::new()),
            Err(expected_error_message)
        );
    }

    #[test]
    fn test_config_parsing_should_error_on_missing_query_string() {
        run_test_missing_arg(vec!["minigrep".to_string()], "Didn't get a query string");
    }

    #[test]
    fn test_config_parsing_should_error_on_missing_filename_arg() {
        run_test_missing_arg(
            vec!["minigrep".to_string(), "some query".to_string()], "Didn't get a file name"
        );
    }

}
