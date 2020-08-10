use crate::command::handler::{CommandHandler, CommandExecutor, ParsedArgMap};
use crate::employee_store::EmployeeStore;
use regex:: Regex;

const MATCH_PATTERN_DESCRIPTION: &'static str = "Delete department (department name)";
const REGEX_PATTERN: &'static str = r"^Delete department (?P<department>.*)$";

pub fn get_handler<E: EmployeeStore>() -> CommandHandler<E> {
    let executor: CommandExecutor<E> = |arg_map: ParsedArgMap, store: &mut E| {
        let department = arg_map.get("department").unwrap();
        info!("Deleting department \"{}\"", department);

        match store.delete_department(department) {
            Ok(dept_info) => {
                info!("Department deleted successfully - \"{}\" (employees {})",
                      department, dept_info.employee_names.join(", "));
                Ok(())
            },
            Err(error_message) => {
                error!("Deletion failed with error: {}", error_message);
                Err("Failed to delete department")
            }
        }
    };

    CommandHandler {
        match_pattern_description: MATCH_PATTERN_DESCRIPTION,
        matcher_regex: Regex::new(REGEX_PATTERN).unwrap(),
        expected_args: vec!["department".to_string()],
        executor
    }
}


#[cfg(test)]
mod tests {
    use crate::employee_store::{MockEmployeeStore, DepartmentInfo};
    use crate::command::HandleCommand;
    use crate::command::handler::CommandHandler;
    use super::get_handler;
    use mockall::predicate::eq;
    use log::Level;

    const MATCHING_COMMAND: &'static str = "Delete department Pie Eating";
    const NON_MATCHING_COMMAND: &'static str = "We are closing the Pie Eating department";

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

    fn run_test_call_executor(
        mock_return_value: Result<DepartmentInfo, String>, expected_handler_result: Result<(), &str>,
        additional_log_body: &str, additional_log_level: Level
    ) {
        testing_logger::setup();

        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_delete_department()
            .times(1)
            .with(eq("Pie Eating".to_string()))
            .return_const(mock_return_value);

        let result = get_handler()
            .execute_command(MATCHING_COMMAND, &mut mock_store);

        assert_eq!(result, expected_handler_result);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 2);
            assert_eq!(captured_logs[0].body, "Deleting department \"Pie Eating\"");
            assert_eq!(captured_logs[0].level, Level::Info);
            assert_eq!(captured_logs[1].body, additional_log_body);
            assert_eq!(captured_logs[1].level, additional_log_level);
        })

    }

    #[test]
    fn test_handles_successful_deletion() {
        let mock_return_value = Ok(
            DepartmentInfo {
                department: "Pie Eating".to_string(),
                employee_names: vec!["Bob".to_string(), "Weebl".to_string()]
            }
        );

        run_test_call_executor(
            mock_return_value,
            Ok(()),
            "Department deleted successfully - \"Pie Eating\" (employees Bob, Weebl)",
            Level::Info
        );
    }

    #[test]
    fn test_handles_failure_to_delete_non_existent_department() {
        run_test_call_executor(
            Err("Could not delete department \"Pie Eating\" - no such department".to_string()),
            Err("Failed to delete department"),
            "Deletion failed with error: Could not delete department \"Pie Eating\" - no such department",
            Level::Error
        );
    }

}