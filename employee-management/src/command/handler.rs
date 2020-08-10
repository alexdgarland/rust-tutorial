mod add_employee;
mod delete_department;
mod delete_employee;
mod list_departments;
mod retrieve_all_employees;
mod retrieve_employees_by_department;

use std::collections::HashMap;

use regex::{Captures, Regex};

use crate::employee_store::EmployeeStore;

use super::HandleCommand;


pub type ParsedArgMap = HashMap<String, String>;
pub type CommandExecutor<E> = fn(ParsedArgMap, &mut E) -> Result<(), &'static str>;

pub fn get_all_handlers<E: EmployeeStore>() -> Vec<CommandHandler<E>> {
    vec![
        add_employee::get_handler(),
        delete_department::get_handler(),
        delete_employee::get_handler(),
        list_departments::get_handler(),
        retrieve_all_employees::get_handler(),
        retrieve_employees_by_department::get_handler(),
    ]
}

static NON_PARSEABLE_ERROR: Result<(), &str> = Err("Could not parse expected args from command by matching expected pattern");

pub struct CommandHandler<E: EmployeeStore> {
    match_pattern_description: &'static str,
    matcher_regex: Regex,
    expected_args: Vec<String>,
    executor: CommandExecutor<E>,
}

impl<E: EmployeeStore> CommandHandler<E> {
    pub fn new(match_pattern_description: &'static str,
               matcher_regex: Regex,
               expected_args: Vec<&str>,
               executor: CommandExecutor<E>,
    ) -> CommandHandler<E> {
        let expected_args_ownable: Vec<String> = expected_args.iter().map(|s| s.to_string()).collect();
        CommandHandler {
            match_pattern_description,
            matcher_regex,
            expected_args: expected_args_ownable,
            executor,
        }
    }
}

fn extract_args(regex: &Regex, expected_args: &Vec<String>, command_text: &str) -> Option<ParsedArgMap> {
    let _captures_to_args = |captures: Captures| -> Option<ParsedArgMap> {
        let mut args_map = ParsedArgMap::new();
        for arg_key in expected_args {
            match captures.name(&arg_key).map(|m| m.as_str().to_string()) {
                Some(arg_value) => {
                    args_map.insert(arg_key.to_string(), arg_value);
                }
                None =>
                    // This should only occur when we are writing and debugging handlers (not at application runtime)
                    //  so a panic seems valid. We're basically asserting that the "soft" type system created
                    //  by the regex pattern and expected args (which we can't check at compile-time) is being followed.
                    panic!("Could not find arg \"{}\" in {:?}", arg_key, captures)
            }
        }
        Some(args_map)
    };
    regex
        .captures(command_text)
        .and_then(_captures_to_args)
}

impl<E: 'static + EmployeeStore> HandleCommand<E> for CommandHandler<E> {
    fn matches_command_text(&self, command_text: &str) -> bool {
        let result = self.matcher_regex.is_match(command_text);
        let result_description = if result { "successfully matched" } else { "did not match" };
        debug!("Command text {} pattern \"{}\"", result_description, self.match_pattern_description);
        result
    }

    fn execute_command(&self, command_text: &str, employee_store: &mut E) -> Result<(), &'static str> {
        match extract_args(&self.matcher_regex, &self.expected_args, command_text) {
            Some(arg_map) =>
                (self.executor)(arg_map.clone(), employee_store),
            None =>
                NON_PARSEABLE_ERROR
        }
    }

    fn describe(&self) -> String {
        self.match_pattern_description.to_string()
    }
}

#[cfg(test)]
mod tests {
    use log::Level;
    use regex::Regex;

    use crate::employee_store::EmployeeStoreImpl;

    use super::{CommandExecutor, CommandHandler, NON_PARSEABLE_ERROR, ParsedArgMap};
    use super::super::HandleCommand;

    static MATCHING_COMMAND: &str = "Do something with value 1 and value 2";
    static NON_MATCHING_COMMAND: &str = "Handle value 1, also value 2";
    static STUB_EXECUTOR_RETURN: Result<(), &str> = Err("Some error occurred according to stub executor");
    static STUB_EXECUTOR: CommandExecutor<EmployeeStoreImpl> = |arg_map: ParsedArgMap, store: &mut EmployeeStoreImpl| {
        assert_eq!(arg_map, get_test_arg_map());
        assert_eq!(*store, EmployeeStoreImpl::new());
        STUB_EXECUTOR_RETURN
    };

    fn get_test_arg_map() -> ParsedArgMap {
        let mut args = ParsedArgMap::new();
        args.insert("arg_1".to_string(), "value 1".to_string());
        args.insert("arg_2".to_string(), "value 2".to_string());
        args
    }

    fn get_test_handler() -> CommandHandler<EmployeeStoreImpl> {
        CommandHandler {
            match_pattern_description: "Do something with (argument 1) and (argument 2)",
            matcher_regex: Regex::new(r"^Do something with (?P<arg_1>.*) and (?P<arg_2>.*)$").unwrap(),
            expected_args: vec!["arg_1".to_string(), "arg_2".to_string()],
            executor: STUB_EXECUTOR,
        }
    }

    fn run_test_against_matcher(command_text: &str, expected_return: bool, expected_log_message: &str) {
        testing_logger::setup();
        assert_eq!(
            get_test_handler().matches_command_text(command_text),
            expected_return
        );
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, expected_log_message);
            assert_eq!(captured_logs[0].level, Level::Debug);
        });
    }

    #[test]
    fn test_matches_command_text_true() {
        run_test_against_matcher(
            MATCHING_COMMAND, true,
            "Command text successfully matched pattern \"Do something with (argument 1) and (argument 2)\"",
        );
    }

    #[test]
    fn test_matches_command_text_false() {
        run_test_against_matcher(
            NON_MATCHING_COMMAND, false,
            "Command text did not match pattern \"Do something with (argument 1) and (argument 2)\"",
        );
    }

    fn run_test_against_executor(command_text: &str, expected_return: Result<(), &str>) {
        assert_eq!(
            get_test_handler().execute_command(command_text, &mut EmployeeStoreImpl::new()),
            expected_return
        );
    }

    #[test]
    fn test_calls_executor_matching_command() {
        run_test_against_executor(MATCHING_COMMAND, STUB_EXECUTOR_RETURN);
    }

    #[test]
    fn test_calls_executor_non_matching_command() {
        run_test_against_executor(NON_MATCHING_COMMAND, NON_PARSEABLE_ERROR);
    }

    #[test]
    #[allow(unused_must_use)]
    #[should_panic(expected = "Could not find arg \"arg_2\" in Captures({0: Some(\"Take value foo\"), \"arg_1\": Some(\"foo\")})")]
    fn test_execution_panics_in_a_descriptive_way_if_expected_args_dont_match_regex_pattern() {
        let handler = CommandHandler {
            match_pattern_description: "",
            matcher_regex: Regex::new(r"^Take value (?P<arg_1>.*)$").unwrap(),
            expected_args: vec!["arg_2".to_string()],
            executor: STUB_EXECUTOR,
        };
        handler.execute_command("Take value foo", &mut EmployeeStoreImpl::new());
    }



}
