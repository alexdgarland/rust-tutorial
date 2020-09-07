use crate::employee_store::EmployeeStore;
use super::HandleCommand;

pub struct CommandDispatcher<E: 'static + EmployeeStore, H: HandleCommand<E>> {
    command_handlers: Vec<H>,
    employee_store: E,
}

pub fn create_dispatcher<E: 'static + EmployeeStore, H: HandleCommand<E>>(command_handlers: Vec<H>, employee_store: E)
    -> CommandDispatcher<E, H>
{
    CommandDispatcher { command_handlers, employee_store }
}

impl<E: 'static + EmployeeStore, H: HandleCommand<E>> CommandDispatcher<E, H> {

    pub fn process_command(&mut self, command_text: &str) -> Result<String, String> {

        debug!("Checking for command matching text \"{}\"", command_text);

        for handler in &self.command_handlers {
            if handler.matches_command_text(command_text) {
                return handler.execute_command(command_text, &mut self.employee_store)
            }
        }

        Err(format!("No matching handler found for command \"{}\"", command_text))
    }

    pub fn get_usage_text(&self) -> String {
        let mut text = "Employee Management - valid command formats:\n".to_string();
        for handler in &self.command_handlers {
            text += &format!(" - \"{}\"\n", &handler.describe());
        }
        text
    }

}


#[cfg(test)]
mod tests {
    use super::CommandDispatcher;
    use crate::employee_store::{MockEmployeeStore, EmployeeStoreImpl};
    use log::Level::Debug;
    use mockall::predicate::eq;
    use crate::command::MockHandleCommand;

    static COMMAND: &str = "Some command";

    type MockHandler = MockHandleCommand<EmployeeStoreImpl>;

    impl MockHandler {

        fn with_match_called_expectation(mut self, return_value: bool) -> MockHandler {
            self
                .expect_matches_command_text()
                .times(1)
                .with(eq(COMMAND))
                .return_const(return_value);
            self
        }

        fn with_execute_called_expectation(mut self, return_value: Result<String, String>) -> MockHandler {
            self
                .expect_execute_command()
                .times(1)
                .with(eq(COMMAND), eq(EmployeeStoreImpl::new()))
                .return_const(return_value);
            self
        }

    }

    fn get_success_result() -> Result<String, String> {
        Ok("Executor succeeded".to_string())
    }

    fn handler_match_expect_executor_called() -> MockHandler {
        MockHandleCommand::new()
            .with_match_called_expectation(true)
            .with_execute_called_expectation(get_success_result())
    }

    fn handler_non_match() -> MockHandler {
        MockHandleCommand::new().with_match_called_expectation(false)
    }

    fn run_test(command_handlers: Vec<MockHandler>, expected_result: Result<String, String>) {
        testing_logger::setup();
        let mut dispatcher = CommandDispatcher { command_handlers, employee_store: EmployeeStoreImpl::new(), };
        assert_eq!(dispatcher.process_command(COMMAND), expected_result);
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, "Checking for command matching text \"Some command\"");
            assert_eq!(captured_logs[0].level, Debug);
        });
    }

    #[test]
    fn test_calls_one_handler_which_matches_command() {
        let command_handlers = vec![handler_match_expect_executor_called()];
        run_test(command_handlers, get_success_result());
    }

    #[test]
    fn test_stops_on_first_matching_handler() {
        let command_handlers = vec![
            handler_non_match(),
            handler_match_expect_executor_called(),
            MockHandler::new(),     // Does not expect to be called
        ];
        run_test(command_handlers, get_success_result());
    }

    #[test]
    fn test_returns_expected_result_for_no_matching_handlers() {
        let command_handlers = vec![handler_non_match()];
        run_test(command_handlers, Err("No matching handler found for command \"Some command\"".to_string()));
    }

    #[test]
    fn test_returns_expected_result_for_failing_command_execution() {
        let command_handlers = vec![
            MockHandleCommand::new()
                .with_match_called_expectation(true)
                .with_execute_called_expectation(Err("Error from the executor".to_string()))
        ];
        run_test(command_handlers, Err("Error from the executor".to_string()));
    }

    #[test]
    fn test_get_usage_text() {

        fn mock_handler(description: &str) -> MockHandleCommand<MockEmployeeStore> {
            let mut handler = MockHandleCommand::new();
            handler.expect_describe().return_const(description.to_string());
            handler
        }

        let dispatcher = CommandDispatcher {
            command_handlers: vec![mock_handler("Description 1"), mock_handler("Description 2")],
            employee_store: MockEmployeeStore::new()
        };

        let expected_text = "Employee Management - valid command formats:\n".to_string() +
            " - \"Description 1\"\n - \"Description 2\"\n";

        assert_eq!(dispatcher.get_usage_text(), expected_text);
    }

}
