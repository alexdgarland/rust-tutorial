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

fn executor_add_employee<E: EmployeeStore>(arg_map: ParsedArgMap, store: &mut E) -> Result<(), &str> {
    store.add_employee(
        arg_map.get("employee_name").unwrap(),
        arg_map.get("department").unwrap()
    );
    Ok(())
}

// pub static ADD_EMPLOYEE_HANDLER: CommandHandler<E: EmployeeStore> = CommandHandler {
//     match_pattern_description: "Add (employee name) to (department name)",
//     matcher: None,
//     executor: None
// }

#[cfg(test)]
mod tests {
    use crate::exercises::employee_management::command::handler::add_employee::MATCHER_ADD_EMPLOYEE;
    use std::collections::HashMap;
    use crate::exercises::employee_management::command::handler::ParsedArgMap;
    use mockall::predicate::eq;
    use crate::exercises::employee_management::employee_store::MockEmployeeStore;

    fn get_arg_map() -> ParsedArgMap {
        let mut map: ParsedArgMap = HashMap::new();
        map.insert("employee_name".to_string(), "Bob Bobertson".to_string());
        map.insert("department".to_string(), "Pie Quality Control".to_string());
        map
    }

    #[test]
    fn test_matcher_handles_matching_pattern() {
        assert_eq!(MATCHER_ADD_EMPLOYEE("Add Bob Bobertson to Pie Quality Control"), Some(get_arg_map()));
    }

    #[test]
    fn test_matcher_handles_non_matching_pattern() {
        assert_eq!(MATCHER_ADD_EMPLOYEE("Add Bob Bobertson into the Pie Quality Control department"), None);
    }

    #[test]
    fn test_executor_calls_expected_method_on_store() {
        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_add_employee()
            .times(1)
            .with(
                eq(String::from("Bob Bobertson")),
                eq(String::from("Pie Quality Control")),
            ).return_const(());

        super::executor_add_employee(get_arg_map(), &mut mock_store);
    }

}