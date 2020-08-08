use super::{ParsedArgMap, CommandHandler, CommandExecutor};
use crate::exercises::employee_management::employee_store::EmployeeStore;
use regex::Regex;

const MATCH_PATTERN_DESCRIPTION: &'static str = "List Departments";
const REGEX_PATTERN: &'static str = r"^List departments$";

pub fn get_handler<E: EmployeeStore>() -> CommandHandler<E> {
    let executor: CommandExecutor<E> = |_arg_map: ParsedArgMap, store: &mut E| {
        info!("Retrieving department list");
        info!("{}", store.list_departments().join(", "));
        Ok(())
    };

    CommandHandler::new(
        MATCH_PATTERN_DESCRIPTION,
        Regex::new(REGEX_PATTERN).unwrap(),
        vec![],
        executor,
    )
}


#[cfg(test)]
mod tests {
    use super::get_handler;
    use crate::exercises::employee_management::employee_store::MockEmployeeStore;
    use log::Level;
    use crate::exercises::employee_management::command::HandleCommand;
    use crate::exercises::employee_management::command::handler::CommandHandler;

    const MATCHING_COMMAND: &str = "List departments";
    const NON_MATCHING_COMMAND: &'static str = "Tell me all the departments now!";

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
    fn test_executor_calls_store_as_expected() {
        testing_logger::setup();
        let list_departments_return = vec![
            "Pie Quality Control".to_string(), "Stealthy Buccaneering".to_string()
        ];

        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_list_departments()
            .times(1)
            .with()
            .return_const(list_departments_return);

        let result = get_handler()
            .execute_command(MATCHING_COMMAND, &mut mock_store);

        assert_eq!(result, Ok(()));

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 2);
            assert_eq!(captured_logs[0].body, "Retrieving department list");
            assert_eq!(captured_logs[0].level, Level::Info);
            assert_eq!(captured_logs[1].body, "Pie Quality Control, Stealthy Buccaneering");
            assert_eq!(captured_logs[1].level, Level::Info);
        });
    }
}