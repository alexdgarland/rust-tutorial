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
                info!("Number of employees found: {}", employees.len());
                info!("{}", employees.join(", "));
                Ok(())
            },
            None => {
                warn!("Department \"{}\" does not exist", department);
                Err("Department does not exist")
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

    const MATCHING_COMMAND: &str = "Retrieve department Pie Quality Control";
    const NON_MATCHING_COMMAND: &'static str = "Tell me who works in Pie Quality Control";

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

    fn run_test_call_executor(mock_return_value: Option<Vec<String>>, expected_handler_result: Result<(), &str>,
                              additional_log_entries: Vec<(&str, Level)>) {
        testing_logger::setup();

        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_retrieve_employees_by_department()
            .times(1)
            .with(eq(String::from("Pie Quality Control")))
            .return_const(mock_return_value);

        let handler_result = get_handler()
            .execute_command(MATCHING_COMMAND, &mut mock_store);

        assert_eq!(handler_result, expected_handler_result);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), additional_log_entries.len() + 1);
            assert_eq!(captured_logs[0].body, "Retrieving employees for department \"Pie Quality Control\"");
            assert_eq!(captured_logs[0].level, Level::Info);
            for (captured, (expected_message, expected_level))
            in captured_logs[1..].iter().zip(&additional_log_entries) {
                assert_eq!(&captured.body, expected_message);
                assert_eq!(&captured.level, expected_level);
            }
        });
    }

    #[test]
    fn test_executor_calls_store_handles_existing_department() {
        run_test_call_executor(Some(vec!["Bob Bobertson".to_string(), "Weebl Bull".to_string()]),
                               Ok(()),
                               vec![
                                   ("Number of employees found: 2", Level::Info),
                                   ("Bob Bobertson, Weebl Bull", Level::Info)
                               ]
        );
    }

    #[test]
    fn test_executor_calls_store_handles_non_existent_department() {
        run_test_call_executor(None,
                               Err("Department does not exist"),
                               vec![("Department \"Pie Quality Control\" does not exist", Level::Warn)]
        );
    }
}