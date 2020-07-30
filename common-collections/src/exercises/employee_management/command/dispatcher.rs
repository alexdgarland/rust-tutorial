use crate::exercises::employee_management::employee_store::EmployeeStore;
use CommandProcessingResult::{Success, NoMatchingHandlerFound, HandlerExecutionFailed};
use super::handler::CommandHandler;

// TODO - rewrite existing command functions to match this struct pattern

pub struct CommandDispatcher<E: 'static + EmployeeStore> {
    command_handlers: Vec<CommandHandler<E>>,
    employee_store: E,
}

#[derive(Debug, Eq, PartialEq)]
pub enum CommandProcessingResult {
    Success,
    NoMatchingHandlerFound,
    HandlerExecutionFailed(String)
}

// TODO - some message like this could be used elsewhere?
// static NO_MATCHES_ERROR_MESSAGE: &str = "No match could be found to execute the submitted text command";

// TODO rewire rest of code to use this dispatcher
impl<E: 'static + EmployeeStore> CommandDispatcher<E> {
    pub fn process_command(&mut self, command_text: &str) -> CommandProcessingResult {

        debug!("Checking for command matching text \"{}\"", command_text);

        for handler in &self.command_handlers {

            match (handler.matcher)(command_text) {
                Some(args) => {
                    debug!("Successfully matched pattern \"{}\"", handler.match_pattern_description);

                    return match (handler.executor)(args, &mut self.employee_store) {
                        Ok(()) =>
                            Success,
                        Err(error_message) => {
                            debug!("Command execution failed with error \"{}\"", error_message);
                            HandlerExecutionFailed(error_message.to_string())
                        }
                    }
                }
                None =>
                    debug!("Did not match pattern \"{}\"", handler.match_pattern_description)
            }

        }

        debug!("No matching handler found");
        NoMatchingHandlerFound
    }
}


#[cfg(test)]
mod tests {
    use super::super::handler::{CommandHandler, CommandExecutor, CommandMatcher};
    use super::{CommandDispatcher, CommandProcessingResult};
    use super::CommandProcessingResult::{Success, NoMatchingHandlerFound, HandlerExecutionFailed};
    use crate::exercises::employee_management::employee_store::MockEmployeeStore;
    use std::collections::HashMap;
    use log::Level::Debug;

    static COMMAND: &str = "Some command";

    fn get_test_args_map() -> HashMap<String, String> {
        let mut args_map = HashMap::new();
        args_map.insert("Some arg name".to_string(), "Some arg value".to_string());
        args_map
    }

    static EXECUTOR_CHECK_ARGS: CommandExecutor<MockEmployeeStore> =
        |supplied_args, _store: &mut MockEmployeeStore| {
            assert_eq!(supplied_args, get_test_args_map(), "Expected executor args were not passed");
            Ok(())
        };

    static EXECUTOR_FAIL_TEST_IF_CALLED: CommandExecutor<MockEmployeeStore> =
        |_supplied_args, _store: &mut MockEmployeeStore| {
            assert!(false, "This executor should never be called");
            Ok(())
        };

    static EXECUTOR_FAIL_EXECUTION: CommandExecutor<MockEmployeeStore> =
        |supplied_args, _store: &mut MockEmployeeStore| {
            assert_eq!(supplied_args, get_test_args_map(), "Expected executor args were not passed");
            Err("Error from the executor")
        };

    static MATCHER_SUCCESSFUL: CommandMatcher = |command_text: &str| {
        assert_eq!(command_text, COMMAND);
        Some(get_test_args_map())
    };

    static MATCHER_UNSUCCESSFUL: CommandMatcher = |command_text: &str| {
        assert_eq!(command_text, COMMAND);
        None
    };

    fn handler_match_expect_executor_called() -> CommandHandler<MockEmployeeStore> {
        CommandHandler {
            match_pattern_description: "Some (thing)",
            matcher: MATCHER_SUCCESSFUL,
            executor: EXECUTOR_CHECK_ARGS,
        }
    }

    fn handler_match_expect_executor_not_called() -> CommandHandler<MockEmployeeStore> {
        CommandHandler {
            match_pattern_description: "Some other pattern for (thing)",
            matcher: MATCHER_SUCCESSFUL,
            executor: EXECUTOR_FAIL_TEST_IF_CALLED,
        }
    }

    fn handler_non_match() -> CommandHandler<MockEmployeeStore> {
        CommandHandler {
            match_pattern_description: "Doesn't match (anything)",
            matcher: MATCHER_UNSUCCESSFUL,
            executor: EXECUTOR_FAIL_TEST_IF_CALLED,
        }
    }

    fn handler_match_executor_will_error() -> CommandHandler<MockEmployeeStore> {
        CommandHandler {
            match_pattern_description: "Some (thing)",
            matcher: MATCHER_SUCCESSFUL,
            executor: EXECUTOR_FAIL_EXECUTION,
        }
    }

    static EXPECTED_FIRST_LOG_LINE: &str = "Checking for command matching text \"Some command\"";
    static SUCCESSFUL_MATCH_LOG_LINE: &str = "Successfully matched pattern \"Some (thing)\"";
    static NON_MATCH_LOG_LINE: &str = "Did not match pattern \"Doesn't match (anything)\"";

    fn run_test(
        command_handlers: Vec<CommandHandler<MockEmployeeStore>>,
        expected_result: CommandProcessingResult,
        expected_log_lines: Vec<&str>,
    ) {
        testing_logger::setup();
        let mut dispatcher = CommandDispatcher { command_handlers, employee_store: MockEmployeeStore::new(), };

        assert_eq!(dispatcher.process_command(COMMAND), expected_result);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), expected_log_lines.len(), "Did not get expected number of log entries");
            for (captured, expected_body) in captured_logs
                .iter()
                .zip(expected_log_lines.iter())
            {
                assert_eq!(captured.level, Debug);
                assert_eq!(&captured.body, expected_body);
            }
        });
    }

    #[test]
    fn test_calls_one_handler_which_matches_command() {
        let command_handlers = vec![handler_match_expect_executor_called()];
        let expected_log_lines = vec![EXPECTED_FIRST_LOG_LINE, SUCCESSFUL_MATCH_LOG_LINE];
        run_test(command_handlers, Success, expected_log_lines);
    }

    #[test]
    fn test_bypasses_non_matching_handler() {
        let command_handlers = vec![
            handler_non_match(),
            handler_match_expect_executor_called(),
        ];
        let expected_log_lines = vec![EXPECTED_FIRST_LOG_LINE, NON_MATCH_LOG_LINE, SUCCESSFUL_MATCH_LOG_LINE];
        run_test(command_handlers, Success, expected_log_lines);
    }

    #[test]
    fn test_stops_on_first_matching_handler() {
        let command_handlers = vec![
            handler_non_match(),
            handler_match_expect_executor_called(),
            handler_match_expect_executor_not_called(),
        ];
        let expected_log_lines = vec![EXPECTED_FIRST_LOG_LINE, NON_MATCH_LOG_LINE, SUCCESSFUL_MATCH_LOG_LINE];
        run_test(command_handlers, Success, expected_log_lines);
    }

    #[test]
    fn test_returns_expected_result_for_no_matching_handlers() {
        let command_handlers = vec![handler_non_match()];
        let expected_log_lines = vec![EXPECTED_FIRST_LOG_LINE, NON_MATCH_LOG_LINE, "No matching handler found"];
        run_test(command_handlers, NoMatchingHandlerFound, expected_log_lines);
    }

    #[test]
    fn test_returns_expected_result_for_failing_command_execution() {
        let command_handlers = vec![handler_match_executor_will_error()];
        let expected_log_lines = vec![
            EXPECTED_FIRST_LOG_LINE,
            SUCCESSFUL_MATCH_LOG_LINE,
            "Command execution failed with error \"Error from the executor\""
        ];
        let expected_result = HandlerExecutionFailed("Error from the executor".to_string());
        run_test(command_handlers, expected_result, expected_log_lines);
    }

}
