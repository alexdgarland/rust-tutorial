use super::{ParsedArgMap, CommandHandler};
use crate::employee_store::EmployeeStore;
use crate::employee_store::EmployeeDeletionResult::{
    SuccessfullyDeleted, NoSuchDepartment, EmployeeNotInDepartment,
};
use regex::Regex;

const MATCH_PATTERN_DESCRIPTION: &'static str = "Delete (employee name) from (department name)";
const REGEX_PATTERN: &'static str = r"^Delete (?P<employee_name>.*) from (?P<department>.*)$";

pub fn get_handler<E: EmployeeStore>() -> CommandHandler<E> {
    let executor = |arg_map: ParsedArgMap, store: &mut E| {
        let employee_name = arg_map.get("employee_name").unwrap();
        let department = arg_map.get("department").unwrap();
        match store.delete_employee(employee_name, department) {
            NoSuchDepartment => {
                info!("Department \"{}\" does not exist", department);
                Err("No such department")
            }
            EmployeeNotInDepartment => {
                info!("Employee \"{}\" does not exist in department \"{}\"", employee_name, department);
                Err("Employee not in department")
            }
            SuccessfullyDeleted => {
                info!("Successfully deleted employee \"{}\" from department \"{}\"", employee_name, department);
                Ok(())
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
    use log::Level;

    const MATCHING_COMMAND: &str = "Delete Bob Bobertson from Pie Quality Control";
    const NON_MATCHING_COMMAND: &'static str = "Bob Bobertson shouldn't be in the Pie Eating department";

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

    fn run_executor_call_test(mock_store_return_value: EmployeeDeletionResult, expected_result: Result<(), &str>,
                              expected_log_entry: &str) {
        testing_logger::setup();
        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_delete_employee()
            .times(1)
            .with(
                eq(String::from("Bob Bobertson")),
                eq(String::from("Pie Quality Control")),
            ).return_const(mock_store_return_value);

        let result = get_handler()
            .execute_command(MATCHING_COMMAND, &mut mock_store);

        assert_eq!(result, expected_result);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].level, Level::Info);
            assert_eq!(captured_logs[0].body, expected_log_entry);
        });
    }

    #[test]
    fn test_executor_call_handles_successful_deletion() {
        run_executor_call_test(
            SuccessfullyDeleted, Ok(()),
            "Successfully deleted employee \"Bob Bobertson\" from department \"Pie Quality Control\"",
        );
    }

    #[test]
    fn test_executor_calls_expected_method_on_store_handles_no_such_department() {
        run_executor_call_test(
            NoSuchDepartment, Err("No such department"),
            "Department \"Pie Quality Control\" does not exist",
        );
    }

    #[test]
    fn test_executor_calls_expected_method_on_store_handles_employee_not_in_department() {
        run_executor_call_test(
            EmployeeNotInDepartment, Err("Employee not in department"),
            "Employee \"Bob Bobertson\" does not exist in department \"Pie Quality Control\"",
        );
    }
}