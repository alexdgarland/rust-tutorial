use crate::command::handler::{CommandHandler, CommandExecutor, ParsedArgMap};
use crate::employee_store::EmployeeStore;
use regex:: Regex;

const MATCH_PATTERN_DESCRIPTION: &'static str = "Delete department (department name)";
const REGEX_PATTERN: &'static str = r"^Delete department (?P<department>.*)$";

pub fn get_handler<E: EmployeeStore>() -> CommandHandler<E> {
    let executor: CommandExecutor<E> = |arg_map: ParsedArgMap, store: &mut E| {
        let department = arg_map.get("department").unwrap();
        info!("Deleting department \"{}\"", department);
        store.delete_department(department).map(
            |dept_info|
                format!(
                    "Department deleted successfully - \"{}\" (employees {})",
                    department, dept_info.employee_names.join(", ")
                )
        )
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
        mock_return_value: Result<DepartmentInfo, String>, expected_handler_result: Result<String, String>
    ) {
        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_delete_department()
            .times(1)
            .with(eq("Pie Eating".to_string()))
            .return_once(move |_dept| mock_return_value);

        let result = get_handler()
            .execute_command(MATCHING_COMMAND, &mut mock_store);

        assert_eq!(result, expected_handler_result);
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
            Ok("Department deleted successfully - \"Pie Eating\" (employees Bob, Weebl)".to_string())
        );
    }

    #[test]
    fn test_handles_failure_to_delete_non_existent_department() {
        run_test_call_executor(
            Err("Something bad happened".to_string()),
            Err("Something bad happened".to_string())
        );
    }

}