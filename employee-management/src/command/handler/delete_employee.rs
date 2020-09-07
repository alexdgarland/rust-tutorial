use super::{ParsedArgMap, CommandHandler};
use crate::employee_store::EmployeeStore;
use crate::employee_store::EmployeeDeletionResult::{
    SuccessfullyDeleted, NoSuchDepartment, EmployeeNotInDepartment,
};
use regex::Regex;
use crate::command::handler::CommandExecutor;

const MATCH_PATTERN_DESCRIPTION: &'static str = "Delete (employee name) from (department name)";
const REGEX_PATTERN: &'static str = r"^Delete (?P<employee_name>.*) from (?P<department>.*)$";

pub fn get_handler<E: EmployeeStore>() -> CommandHandler<E> {

    let executor: CommandExecutor<E> = |arg_map: ParsedArgMap, store: &mut E| {
        let employee_name = arg_map.get("employee_name").unwrap();
        let department = arg_map.get("department").unwrap();

        match store.delete_employee(employee_name, department) {
            NoSuchDepartment => {
                Err(format!("Department \"{}\" does not exist", department))
            }
            EmployeeNotInDepartment => {
                Err(format!("Employee \"{}\" does not exist in department \"{}\"", employee_name, department))
            }
            SuccessfullyDeleted => {
                Ok(format!("Successfully deleted employee \"{}\" from department \"{}\"", employee_name, department))
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
    use crate::employee_store::{MockEmployeeStore, EmployeeDeletionResult};
    use crate::employee_store::EmployeeDeletionResult::{
        SuccessfullyDeleted, NoSuchDepartment, EmployeeNotInDepartment,
    };

    const MATCHING_COMMAND: &str = "Delete Bob from Pie QC";
    const NON_MATCHING_COMMAND: &'static str = "Bob shouldn't be in the Pie Eating department";

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

    fn run_executor_call_test(
        mock_store_return_value: EmployeeDeletionResult, expected_result: Result<String, String>
    ) {
        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_delete_employee()
            .times(1)
            .with(
                eq(String::from("Bob")),
                eq(String::from("Pie QC")),
            ).return_once(move |_emp, _dept| mock_store_return_value);

        let result = get_handler()
            .execute_command(MATCHING_COMMAND, &mut mock_store);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_executor_call_handles_successful_deletion() {
        run_executor_call_test(
            SuccessfullyDeleted,
            Ok("Successfully deleted employee \"Bob\" from department \"Pie QC\"".to_string()),
        );
    }

    #[test]
    fn test_executor_calls_expected_method_on_store_handles_no_such_department() {
        run_executor_call_test(
            NoSuchDepartment,
            Err("Department \"Pie QC\" does not exist".to_string())
        );
    }

    #[test]
    fn test_executor_calls_expected_method_on_store_handles_employee_not_in_department() {
        run_executor_call_test(
            EmployeeNotInDepartment,
            Err("Employee \"Bob\" does not exist in department \"Pie QC\"".to_string())
        );
    }
}