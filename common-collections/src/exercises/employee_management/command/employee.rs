pub mod add_employee;
pub mod delete_employee;

use regex::{Captures, Regex};

pub struct EmployeeCommandParameters {
    employee_name: String,
    department: String
}

fn extract_fields(captures: Captures) -> Option<(Option<String>, Option<String>)> {
    let extract = |key: &str|
        captures
            .name(key)
            .map(|m| m.as_str().to_string());
    Some((extract("employee_name"), extract("department")))
}

/// Try matching command against a regex, specifically of such a format
/// that we expect to be able to extract elements for employee_name and department
pub fn parse_employee_command(command: &str, command_regex: &Regex) -> Option<EmployeeCommandParameters>
{
    match command_regex
        .captures(command)
        .and_then(extract_fields)
    {
        Some((Some(employee_name), Some(department))) => {
            Some(EmployeeCommandParameters { employee_name, department })
        }
        _ =>
            None
    }
}
