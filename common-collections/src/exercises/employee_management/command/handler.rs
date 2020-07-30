mod add_employee;

use std::collections::HashMap;
use crate::exercises::employee_management::employee_store::EmployeeStore;

pub type ParsedArgMap = HashMap<String, String>;
pub type CommandMatcher = fn(&str) -> Option<ParsedArgMap>;
pub type CommandExecutor<E> = fn(ParsedArgMap, &mut E) -> Result<(), &'static str>;

pub struct CommandHandler<E: EmployeeStore> {
    // description field is used simply to show dispatcher usage options
    pub match_pattern_description: &'static str,
    pub matcher: CommandMatcher,
    pub executor: CommandExecutor<E>,
}
