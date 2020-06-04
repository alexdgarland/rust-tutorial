use crate::exercises::employee_management::text_command_executor::TextCommandExecutor;

static NO_MATCHES_ERROR: Result<(), &str> = Err("No match could be found for submitted text command");

struct TextCommandDispatcher {
    command_executors: Vec<Box<dyn TextCommandExecutor>>
}

impl TextCommandDispatcher {
    pub fn process(&mut self, text_command: &str) -> Result<(), &str> {
        for executor in &mut self.command_executors {
            match executor.try_execute(text_command) {
                Ok(()) =>
                    return Ok(()),
                Err(executor_error_message) =>
                    info!("{}", executor_error_message)
            }
        }
        NO_MATCHES_ERROR
    }

}

#[cfg(test)]
mod text_command_dispatcher_tests {
    use mockall::predicate::eq;

    use super::NO_MATCHES_ERROR;
    use super::super::text_command_executor::{MockTextCommandExecutor, TextCommandExecutor};
    use super::TextCommandDispatcher;

    static COMMAND: &str = "Some command";
    static ERROR_RETURN_MESSAGE: &str = "This mock executor couldn't handle the text command";
    static ERROR_RETURN: Result<(), &str> = Err(ERROR_RETURN_MESSAGE);

    fn mock_executor(expected_call_count: usize, result: Result<(), &'static str>)
                     -> Box<dyn TextCommandExecutor>
    {
        let mut mock_executor = MockTextCommandExecutor::new();
        mock_executor.expect_try_execute().times(expected_call_count).with(eq(COMMAND)).return_const(result);
        Box::new(mock_executor)
    }

    fn assert_error_log_entries(expected_length: usize) {
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), expected_length);
            for entry in captured_logs {
                assert_eq!(entry.body, ERROR_RETURN_MESSAGE);
                assert_eq!(entry.level, log::Level::Info);
            };
        });
    }

    #[test]
    fn test_calls_one_executor_with_matching_command() {
        let mut dispatcher = TextCommandDispatcher {
            command_executors: vec![mock_executor(1, Ok(())),]
        };
        assert_eq!(dispatcher.process(COMMAND), Ok(()));
    }

    #[test]
    fn test_bypasses_non_matching_executor() {
        testing_logger::setup();
        let mut dispatcher = TextCommandDispatcher {
            command_executors: vec![
                mock_executor(1, ERROR_RETURN),
                mock_executor(1, Ok(())),
            ]
        };
        assert_eq!(dispatcher.process(COMMAND), Ok(()));
        assert_error_log_entries(1);
    }

    #[test]
    fn test_stops_on_first_matching_executor() {
        let mut dispatcher = TextCommandDispatcher {
            command_executors: vec![
                mock_executor(1, Ok(())),
                mock_executor(0, Ok(())),
                mock_executor(0, ERROR_RETURN),
            ]
        };
        assert_eq!(dispatcher.process(COMMAND), Ok(()));
    }

    #[test]
    fn test_returns_error_if_no_matching_executors() {
        testing_logger::setup();
        let mut dispatcher = TextCommandDispatcher {
            command_executors: vec![
                mock_executor(1, ERROR_RETURN),
                mock_executor(1, ERROR_RETURN),
            ]
        };
        assert_eq!(dispatcher.process(COMMAND), NO_MATCHES_ERROR);
        assert_error_log_entries(2);
    }

}
