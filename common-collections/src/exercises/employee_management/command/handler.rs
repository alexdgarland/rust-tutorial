mod add_employee;

use std::collections::HashMap;
use crate::exercises::employee_management::employee_store::EmployeeStore;
use mockall_derive::automock;
use regex::{Regex, Captures};

pub type ParsedArgMap = HashMap<String, String>;
pub type CommandMatcher = fn(&str) -> Option<ParsedArgMap>; // TODO - delete once all usages removed
pub type CommandExecutor<E> = fn(ParsedArgMap, &mut E) -> Result<(), &'static str>;

#[automock]
pub trait HandleCommand<E: 'static + EmployeeStore> {
    fn matches_command_text(&self, command_text: &str) -> bool;
    // When succeeds in struct impl - log  debug!("Successfully matched pattern \"{}\"", handler.match_pattern_description);
    // When fails -     debug!("Did not match pattern \"{}\"", handler.match_pattern_description)
    fn execute_command(&self, command_text: &str, employee_store: &mut E) -> Result<(), &'static str>;
}

pub struct CommandHandler<E: EmployeeStore> {
    // description field is used simply to show dispatcher usage options
    pub match_pattern_description: &'static str,
    matcher_regex: Regex,
    expected_args: Vec<String>,
    executor: CommandExecutor<E>,
}

impl<E: EmployeeStore> CommandHandler<E> {

    fn extract_args(self, command_text: &str) -> Option<ParsedArgMap> {
        let _captures_to_args = |captures: Captures| -> Option<ParsedArgMap> {
            let mut args_map = ParsedArgMap::new();
            for arg_key in self.expected_args.clone() {
                let arg_value = captures.name(&arg_key).map(|m| m.as_str().to_string()).unwrap();
                args_map.insert(arg_key.to_string(), arg_value);
            }
            Some(args_map)
        };

        self.matcher_regex
            .captures(command_text)
            .and_then((_captures_to_args))
    }

}

impl<E: 'static + EmployeeStore> HandleCommand<E> for CommandHandler<E> {
    fn matches_command_text(&self, command_text: &str) -> bool {
        self.matcher_regex.is_match(command_text)
    }

    // TODO - implement method under TDD
    fn execute_command(&self, command_text: &str, employee_store: &mut E) -> Result<(), &'static str> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {

    use super::CommandHandler;
    use regex::Regex;
    use crate::exercises::employee_management::employee_store::EmployeeStoreImpl;
    use crate::exercises::employee_management::command::handler::{CommandExecutor, ParsedArgMap, HandleCommand};

    fn get_test_handler() -> CommandHandler<EmployeeStoreImpl> {
        CommandHandler {
            match_pattern_description: "",
            matcher_regex: Regex::new(r"^Add (?P<employee_name>.*) to (?P<department>.*)$").unwrap(),
            expected_args: vec!["employee_name".to_string(), "department".to_string()],
            executor: |_command: ParsedArgMap, _store: &mut EmployeeStoreImpl| Ok(())
        }
    }

    #[test]
    fn test_extract_args() {

        let actual_args = get_test_handler()
            .extract_args("Add Bob Bobertson to Pie Quality Control");

        let mut expected_args = ParsedArgMap::new();
        expected_args.insert("employee_name".to_string(), "Bob Bobertson".to_string());
        expected_args.insert("department".to_string(), "Pie Quality Control".to_string());

        assert_eq!(actual_args, Some(expected_args));

    }

    #[test]
    fn test_matches_command_text_true() {
        assert!(get_test_handler().matches_command_text("Add Bob Bobertson to Pie Quality Control"));
    }

    #[test]
    fn test_matches_command_text_false() {
        assert_eq!(get_test_handler().matches_command_text("Add Bob into some department"), false);
    }

}
