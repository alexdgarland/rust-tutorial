use super::{ParsedArgMap, CommandHandler, CommandExecutor};
use crate::employee_store::EmployeeStore;
use regex::Regex;

const MATCH_PATTERN_DESCRIPTION: &'static str = "Add (employee name) to (department name)";
const REGEX_PATTERN: &'static str = r"^Add (?P<employee_name>.*) to (?P<department>.*)$";

pub fn get_handler<E: EmployeeStore>() -> CommandHandler<E> {
    let executor: CommandExecutor<E> = |arg_map: ParsedArgMap, store: &mut E| {
        let employee_name = arg_map.get("employee_name").unwrap();
        let department = arg_map.get("department").unwrap();
        match store
            .retrieve_employees_by_department(department)
            .map(|employees| employees.iter().any(|e| e == employee_name))
        {
            // This check could in some cases be handled as (e.g.) a unique constraint on underlying data store,
            // but let's assume we want to do this as a business logic concern in this layer independent of storage impl
            Some(true) => {
                Err(format!(
                    "Employee \"{}\" already exists in department \"{}\" and cannot be added", employee_name, department)
                )
            },
            _ => {
                // TODO - if this were connecting to an actual database it would be able to error -
                //  do we want to allow for that case?
                store.add_employee(employee_name, department);
                Ok(format!("Successfully added employee \"{}\" to department \"{}\"", employee_name, department))
            }
        }
    };

    CommandHandler::new(
        MATCH_PATTERN_DESCRIPTION,
        Regex::new(REGEX_PATTERN).unwrap(),
        vec!["employee_name", "department"],
        executor,
    )
}


#[cfg(test)]
mod tests {
    use super::get_handler;
    use crate::command::HandleCommand;
    use crate::command::handler::CommandHandler;
    use mockall::predicate::eq;
    use crate::employee_store::MockEmployeeStore;

    const MATCHING_COMMAND: &str = "Add Bob to Pie QC";
    const NON_MATCHING_COMMAND: &'static str = "Add Bob into the Pie Eating department";

    fn run_test_against_matcher(command_text: &str, expected_return: bool) {
        let test_handler: CommandHandler<MockEmployeeStore> = get_handler();
        assert_eq!(test_handler.matches_command_text(command_text), expected_return)
    }

    #[test]
    fn test_matcher_handles_matching_pattern() {
        run_test_against_matcher(MATCHING_COMMAND, true);
    }

    #[test]
    fn test_matcher_handles_non_matching_pattern() {
        run_test_against_matcher(NON_MATCHING_COMMAND, false);
    }

    #[test]
    fn test_executor_calls_expected_method_on_store() {
        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_retrieve_employees_by_department()
            .times(1)
            .with(eq(String::from("Pie QC")))
            .return_const(None);
        mock_store
            .expect_add_employee()
            .times(1)
            .with(
                eq("Bob".to_string()),
                eq("Pie QC".to_string()),
            ).return_const(());

        let result = get_handler()
            .execute_command(MATCHING_COMMAND, &mut mock_store);

        assert_eq!(
            result,
            Ok("Successfully added employee \"Bob\" to department \"Pie QC\"".to_string())
        );
    }

    #[test]
    fn test_executor_errors_without_adding_if_existing_employee_added_to_same_department() {
        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_retrieve_employees_by_department()
            .times(1)
            .with(eq("Pie QC".to_string()))
            .return_const(Some(vec!["Bob".to_string()]));
        mock_store
            .expect_add_employee()
            .times(0);

        let result = get_handler()
            .execute_command(MATCHING_COMMAND, &mut mock_store);

        assert_eq!(
            result,
            Err("Employee \"Bob\" already exists in department \"Pie QC\" and cannot be added".to_string())
        );
    }
}