use super::{ParsedArgMap, CommandHandler, CommandExecutor};
use crate::employee_store::EmployeeStore;
use regex::Regex;

const MATCH_PATTERN_DESCRIPTION: &'static str = "Retrieve department (department name)";
const REGEX_PATTERN: &'static str = r"^Retrieve department (?P<department>.*)$";

pub fn get_handler<E: EmployeeStore>() -> CommandHandler<E> {
    let executor: CommandExecutor<E> = |arg_map: ParsedArgMap, store: &mut E| {
        let department = arg_map.get("department").unwrap();
        info!("Retrieving employees for department \"{}\"", department);
        match store.retrieve_employees_by_department(department) {
            Some(employees) => {
                info!("{}", employees.join(", "));
                Ok(format!("Successfully found {} employees in department \"{}\"", employees.len(), department))
            },
            None => {
                Err(format!("Department \"{}\" does not exist", department))
            }
        }
    };

    CommandHandler::new(
        MATCH_PATTERN_DESCRIPTION,
        Regex::new(REGEX_PATTERN).unwrap(),
        vec!["department"],
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
    use log::Level;

    const MATCHING_COMMAND: &str = "Retrieve department Pie QC";
    const NON_MATCHING_COMMAND: &'static str = "Tell me who works in Pie QC";

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
        mock_return_value: Option<Vec<String>>,
        expected_handler_result: Result<String, String>,
        additional_log_entry: Option<(&str, Level)>
    ) {
        testing_logger::setup();

        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_retrieve_employees_by_department()
            .times(1)
            .with(eq(String::from("Pie QC")))
            .return_const(mock_return_value);

        let handler_result = get_handler()
            .execute_command(MATCHING_COMMAND, &mut mock_store);

        assert_eq!(handler_result, expected_handler_result);

        testing_logger::validate(|captured_logs| {
            // TODO there is a bit much switching logic here for a simple test - maybe clean up
            assert_eq!(captured_logs.len(), if additional_log_entry.is_some() { 2 } else { 1 });
            assert_eq!(captured_logs[0].body, "Retrieving employees for department \"Pie QC\"");
            assert_eq!(captured_logs[0].level, Level::Info);
            if let Some((expected_message, expected_level)) = additional_log_entry {
                assert_eq!(captured_logs[1].body, expected_message);
                assert_eq!(captured_logs[1].level, expected_level);
            }
        });
    }

    #[test]
    fn test_executor_calls_store_handles_existing_department() {
        run_test_call_executor(
            Some(vec!["Bob Bobertson".to_string(), "Weebl Bull".to_string()]),
            Ok("Successfully found 2 employees in department \"Pie QC\"".to_string()),
            Some(("Bob Bobertson, Weebl Bull", Level::Info))
        );
    }

    #[test]
    fn test_executor_calls_store_handles_non_existent_department() {
        run_test_call_executor(
            None,
            Err("Department \"Pie QC\" does not exist".to_string()),
            None
        );
    }
}