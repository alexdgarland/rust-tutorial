mod add_employee;

use std::collections::HashMap;
use crate::exercises::employee_management::employee_store::EmployeeStore;
use mockall_derive::automock;
use regex::{Regex, Captures};

pub type ParsedArgMap = HashMap<String, String>;
pub type CommandMatcher = fn(&str) -> Option<ParsedArgMap>;
pub type CommandExecutor<E> = fn(&str, &mut E) -> Result<(), &'static str>;

#[automock]
pub trait HandleCommand<E: 'static + EmployeeStore> {
    fn matches_command_text(&self, command_text: &str) -> bool;
    // When succeeds in struct impl - log  debug!("Successfully matched pattern \"{}\"", handler.match_pattern_description);
    // When fails -     debug!("Did not match pattern \"{}\"", handler.match_pattern_description)
    fn execute(&self, command_text: &str, employee_store: &mut E) -> Result<(), &'static str>;
}

pub struct CommandHandler<E: EmployeeStore> {
    // description field is used simply to show dispatcher usage options
    pub match_pattern_description: &'static str,
    pub matcher_regex: Regex,
    pub executor: CommandExecutor<E>,
}

impl<E: EmployeeStore> CommandHandler<E> {

    // fn _captures_to_args(captures: Captures) -> Option<ParsedArgMap> {
    //     let mut args_map: ParsedArgMap = HashMap::new();
    //     for arg_key in expected_args {
    //         let arg_value = captures
    //             .name(arg_key)
    //             .map(|m| m.as_str().to_string())
    //             .unwrap();
    //         args_map.insert(arg_key.to_string(), arg_value);
    //     }
    //     Some(args_map)
    // }

    // TODO - does this need testing directly or can it be done implicitly via the execute method?
    fn extract_args(self, command_text: &str) -> Option<ParsedArgMap> {
        self.matcher_regex
            .captures(command_text)
            // TODO - once under test, replace inline lambda with fn above?
            .and_then(|captures| {
                let mut args_map: ParsedArgMap = HashMap::new();
                for arg_key in expected_args {
                    let arg_value = captures
                        .name(arg_key)
                        .map(|m| m.as_str().to_string())
                        .unwrap();
                    args_map.insert(arg_key.to_string(), arg_value);
                }
                Some(args_map)
            })
    }

}

// TODO - implement the methods under TDD
impl HandleCommand<E: EmployeeStore> for CommandHandler<E> {
    fn matches_command_text(&self, command_text: &str) -> bool {
        unimplemented!()
    }

    fn execute(&self, command_text: &str, employee_store: &mut _) -> Result<(), &'static str> {
        unimplemented!()
    }
}

