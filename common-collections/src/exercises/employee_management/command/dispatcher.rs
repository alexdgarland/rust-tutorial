use crate::exercises::employee_management::employee_store::EmployeeStore;
use CommandProcessingResult::{Success, NoMatchingHandlerFound, HandlerExecutionFailed};
use super::handler::{CommandHandler, HandleCommand};

// TODO - rewrite existing command functions to match this struct pattern

pub struct CommandDispatcher<E: 'static + EmployeeStore, H: HandleCommand<E>> {
    command_handlers: Vec<H>,
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
impl<E: 'static + EmployeeStore, H: HandleCommand<E>> CommandDispatcher<E, H> {

    pub fn process_command(&mut self, command_text: &str) -> CommandProcessingResult {

        debug!("Checking for command matching text \"{}\"", command_text);

        for handler in &self.command_handlers {
            if handler.matches_command_text(command_text) {
                return match handler.execute(command_text, &mut self.employee_store) {
                    Ok(()) =>
                        Success,
                    Err(error_message) => {
                        debug!("Command execution failed with error \"{}\"", error_message);
                        HandlerExecutionFailed(error_message.to_string())
                    }
                }
            }
        }

        debug!("No matching handler found");
        NoMatchingHandlerFound

    }
}


#[cfg(test)]
mod tests {
    use super::super::handler::{CommandHandler, CommandExecutor, CommandMatcher, MockHandleCommand};
    use super::{CommandDispatcher, CommandProcessingResult};
    use super::CommandProcessingResult::{Success, NoMatchingHandlerFound, HandlerExecutionFailed};
    use crate::exercises::employee_management::employee_store::{MockEmployeeStore, EmployeeStoreImpl};
    use std::collections::HashMap;
    use log::Level::Debug;
    use mockall::predicate::eq;
    use crate::exercises::employee_management::command::handler::HandleCommand;

    static COMMAND: &str = "Some command";

    type MockHandler = MockHandleCommand<EmployeeStoreImpl>;

    impl MockHandler {

        fn with_match_is_called_expectation(mut self, return_value: bool) -> MockHandler {
            self
                .expect_matches_command_text()
                .times(1)
                .with(eq(COMMAND))
                .return_const(return_value);
            self
        }

        fn with_match_not_called_expectation(mut self) -> MockHandler {
            self.expect_matches_command_text().times(0);
            self
        }

        fn with_execute_is_called_expectation(mut self, return_value: Result<(), &'static str>) -> MockHandler {
            self
                .expect_execute()
                .times(1)
                .with(eq(COMMAND), eq(EmployeeStoreImpl::new()))
                .return_const(return_value);
            self
        }

        fn with_execute_not_called_expectation(mut self) -> MockHandler {
            self.expect_execute().times(0);
            self
        }

    }

    fn handler_match_expect_executor_called() -> MockHandleCommand<EmployeeStoreImpl> {
        MockHandleCommand::new()
            .with_match_is_called_expectation(true)
            .with_execute_is_called_expectation(Ok(()))
    }

    fn handler_match_expect_not_called() -> MockHandleCommand<EmployeeStoreImpl> {
        MockHandleCommand::new()
            .with_match_not_called_expectation()
            .with_execute_not_called_expectation()
    }

    fn handler_non_match() -> MockHandleCommand<EmployeeStoreImpl> {
        MockHandleCommand::new()
            .with_match_is_called_expectation(false)
            .with_execute_not_called_expectation()
    }

    fn handler_match_executor_will_error() -> MockHandleCommand<EmployeeStoreImpl> {
        MockHandleCommand::new()
            .with_match_is_called_expectation(true)
            .with_execute_is_called_expectation(Err("Error from the executor"))
    }

    static EXPECTED_FIRST_LOG_LINE: &str = "Checking for command matching text \"Some command\"";

    fn run_test(
        command_handlers: Vec<MockHandleCommand<EmployeeStoreImpl>>,
        expected_result: CommandProcessingResult,
        expected_log_lines: Vec<&str>,
    ) {
        testing_logger::setup();
        let mut dispatcher = CommandDispatcher { command_handlers, employee_store: EmployeeStoreImpl::new(), };

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
        let expected_log_lines = vec![EXPECTED_FIRST_LOG_LINE];
        run_test(command_handlers, Success, expected_log_lines);
    }

    #[test]
    fn test_bypasses_non_matching_handler() {
        let command_handlers = vec![
            handler_non_match(),
            handler_match_expect_executor_called(),
        ];
        let expected_log_lines = vec![EXPECTED_FIRST_LOG_LINE];
        run_test(command_handlers, Success, expected_log_lines);
    }

    #[test]
    fn test_stops_on_first_matching_handler() {
        let command_handlers = vec![
            handler_non_match(),
            handler_match_expect_executor_called(),
            handler_match_expect_not_called(),
        ];
        let expected_log_lines = vec![EXPECTED_FIRST_LOG_LINE];
        run_test(command_handlers, Success, expected_log_lines);
    }

    #[test]
    fn test_returns_expected_result_for_no_matching_handlers() {
        let command_handlers = vec![handler_non_match()];
        let expected_log_lines = vec![EXPECTED_FIRST_LOG_LINE, "No matching handler found"];
        run_test(command_handlers, NoMatchingHandlerFound, expected_log_lines);
    }

    #[test]
    fn test_returns_expected_result_for_failing_command_execution() {
        let command_handlers = vec![handler_match_executor_will_error()];
        let expected_log_lines = vec![
            EXPECTED_FIRST_LOG_LINE,
            "Command execution failed with error \"Error from the executor\""
        ];
        let expected_result = HandlerExecutionFailed("Error from the executor".to_string());
        run_test(command_handlers, expected_result, expected_log_lines);
    }

}
