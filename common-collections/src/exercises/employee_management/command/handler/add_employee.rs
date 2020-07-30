use super::{ParsedArgMap, CommandMatcher, CommandExecutor, CommandHandler};
use crate::exercises::employee_management::employee_store::EmployeeStore;
use regex::Regex;
use std::collections::HashMap;

// TODO - looks like might be able to factor this out to a function which take expected_args and regex_pattern
//  and returns an appropriate HashMap, possibly even an impl method on a struct
//  which takes these as (private) fields rather than a public matcher
static MATCHER_ADD_EMPLOYEE: CommandMatcher = |command_text| {
    let expected_args = vec!["employee_name", "department"];
    let regex_pattern = r"^Add (?P<employee_name>.*) to (?P<department>.*)$";
    // TODO - regex is getting both initialised and matched against every time it's called
    //  It should be possible (if we consolidate to one set of matching code)
    //  to save the regex and-or the match results if we use a mutable struct?
    //  Although not sure if actually want to do that rather than maintaining immutability?
    Regex::new(regex_pattern)
        .unwrap()
        .captures(command_text)
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
};


// pub static ADD_EMPLOYEE_HANDLER: CommandHandler<E: EmployeeStore> = CommandHandler {
//     match_pattern_description: "Add (employee name) to (department name)",
//     matcher: None,
//     executor: None
// }

#[cfg(test)]
mod tests {
    use crate::exercises::employee_management::command::handler::add_employee::MATCHER_ADD_EMPLOYEE;
    use std::collections::HashMap;

    #[test]
    fn test_matcher_handles_matching_pattern() {
        let mut expected_map = HashMap::new();
        expected_map.insert("employee_name".to_string(), "Bob Bobertson".to_string());
        expected_map.insert("department".to_string(), "Pie Quality Control".to_string());
        assert_eq!(MATCHER_ADD_EMPLOYEE("Add Bob Bobertson to Pie Quality Control"), Some(expected_map));
    }

    #[test]
    fn test_matcher_handles_non_matching_pattern() {
        assert_eq!(MATCHER_ADD_EMPLOYEE("Add Bob Bobertson into the Pie Quality Control department"), None);
    }

}