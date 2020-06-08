use crate::exercises::employee_management::command_executor::CommandExecutor;
use crate::exercises::employee_management::employee_store::{EmployeeStore, EmployeeStoreImpl};

static NO_MATCHES_ERROR: Result<(), &str> = Err("No match could be found to execute the submitted text command");

struct CommandDispatcher<S: 'static + EmployeeStore, C: CommandExecutor> {
    command_executors: Vec<C>,  // TODO - this may actually need to be trait objects to allow for different types to exist in the vector?
    employee_store: S
}
// TODO - related to the above, also a question whether we want to switch
// TODO     to using free functions of a shared type (behind an alias) rather than traits -
// TODO     however to test these properly we may need to switch mocking library to Mocktopus
// TODO     (see https://asomers.github.io/mock_shootout/)

impl<S: 'static + EmployeeStore , C: CommandExecutor> CommandDispatcher<S, C> {
    pub fn process(&mut self, text_command: &str) -> Result<(), &str> {
        for executor in &self.command_executors {
            match executor.try_execute(text_command, &mut self.employee_store) {
                Ok(()) =>
                    return Ok(()),
                Err(executor_error_message) =>
                    info!("{}", executor_error_message)
            }
        }
        NO_MATCHES_ERROR
    }

}

fn create_dispatcher<C: CommandExecutor>(command_executors: Vec<C>) -> CommandDispatcher<EmployeeStoreImpl, C>{
    CommandDispatcher {
        command_executors,
        employee_store: EmployeeStoreImpl::new()
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use super::NO_MATCHES_ERROR;
    use super::super::command_executor::MockCommandExecutor;
    use super::CommandDispatcher;
    use crate::exercises::employee_management::employee_store::{EmployeeStore, EmployeeStoreImpl};

    static COMMAND: &str = "Some command";
    static ERROR_RETURN_MESSAGE: &str = "This mock executor couldn't handle the text command";
    static ERROR_RETURN: Result<(), &str> = Err(ERROR_RETURN_MESSAGE);

    fn mock_employee_store() -> EmployeeStoreImpl {
        let mut store = EmployeeStoreImpl::new();
        store.add_employee(&"Some name".to_string(), &"Some department".to_string());
        store
    }

    fn mock_executor(expected_call_count: usize, result: Result<(), &'static str>) -> MockCommandExecutor
    {
        let mut mock_executor = MockCommandExecutor::new();
        mock_executor
            .expect_try_execute()
            .times(expected_call_count)
            .with(eq(COMMAND), eq(mock_employee_store()))
            .return_const(result);
        mock_executor
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
        let mut dispatcher = CommandDispatcher {
            command_executors: vec![mock_executor(1, Ok(())),],
            employee_store: mock_employee_store()
        };
        assert_eq!(dispatcher.process(COMMAND), Ok(()));
    }

    #[test]
    fn test_bypasses_non_matching_executor() {
        testing_logger::setup();
        let mut dispatcher = CommandDispatcher {
            command_executors: vec![
                mock_executor(1, ERROR_RETURN),
                mock_executor(1, Ok(())),
            ],
            employee_store: mock_employee_store()
        };
        assert_eq!(dispatcher.process(COMMAND), Ok(()));
        assert_error_log_entries(1);
    }

    #[test]
    fn test_stops_on_first_matching_executor() {
        let mut dispatcher = CommandDispatcher {
            command_executors: vec![
                mock_executor(1, Ok(())),
                mock_executor(0, Ok(())),
                mock_executor(0, ERROR_RETURN),
            ],
            employee_store: mock_employee_store()
        };
        assert_eq!(dispatcher.process(COMMAND), Ok(()));
    }

    #[test]
    fn test_returns_error_if_no_matching_executors() {
        testing_logger::setup();
        let mut dispatcher = CommandDispatcher {
            command_executors: vec![
                mock_executor(1, ERROR_RETURN),
                mock_executor(1, ERROR_RETURN),
            ],
            employee_store: mock_employee_store()
        };
        assert_eq!(dispatcher.process(COMMAND), NO_MATCHES_ERROR);
        assert_error_log_entries(2);
    }

}
