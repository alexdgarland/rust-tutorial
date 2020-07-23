mod employee;
mod list_departments;
mod retrieve_by_department;
mod retrieve_all;

use std::result::Result;

use employee::add_employee::add_employee;
use employee::delete_employee::delete_employee;
use list_departments::list_departments;
use retrieve_all::retrieve_all;
use retrieve_by_department::retrieve_by_department;

use crate::exercises::employee_management::employee_store::{EmployeeStore, EmployeeStoreImpl};

static NO_MATCHES_ERROR_MESSAGE: &str = "No match could be found to execute the submitted text command";

pub type Executor<E, R> = fn(&str, &mut E) -> Result<R, &'static str>;

fn get_all_commands() -> Vec<Executor<EmployeeStoreImpl, ()>> {
    vec![
        add_employee,
        delete_employee,
        list_departments,
        retrieve_all,
        retrieve_by_department,
    ]
}

pub struct Dispatcher<E: 'static + EmployeeStore, R> {
    command_executors: Vec<Executor<E, R>>,
    employee_store: E,
}

impl<E: 'static + EmployeeStore, R> Dispatcher<E, R> {
    pub fn process(&mut self, text_command: &str) -> Result<R, &str> {
        debug!("Checking for command matching text \"{}\"", text_command);
        for executor in &self.command_executors {
            match executor(text_command, &mut self.employee_store) {
                // TODO - would be nice to be able to debug-message as soon as the command is matched,
                // but to do that would need to a class-like structure where we can call the command matching
                // before executing the actual operation (or maybe some other kind of refactoring)
                Ok(r) =>
                    return Ok(r),
                Err(executor_error_message) =>
                    debug!("{}", executor_error_message)
            }
        }
        Err(NO_MATCHES_ERROR_MESSAGE)
    }
}

pub fn create_dispatcher() -> Dispatcher<EmployeeStoreImpl, ()> {
    Dispatcher {
        command_executors: get_all_commands(),
        employee_store: EmployeeStoreImpl::new(),
    }
}

#[cfg(test)]
mod tests {
    use log::Level::Debug;
    use testing_logger::CapturedLog;

    use crate::exercises::employee_management::command::NO_MATCHES_ERROR_MESSAGE;
    use crate::exercises::employee_management::employee_store::{EmployeeStore, EmployeeStoreImpl};

    use super::Dispatcher;

    static COMMAND: &str = "Some command";
    static EXECUTOR_SUCCESS_MESSAGE_ONE: &str = "This first executor handled the command okay";
    static EXECUTOR_SUCCESS_MESSAGE_TWO: &str = "This second executor handled the command okay";
    static EXECUTOR_ERROR_MESSAGE: &str = "This executor couldn't handle the text command";

    fn mock_employee_store() -> EmployeeStoreImpl {
        let mut store = EmployeeStoreImpl::new();
        store.add_employee(&"Some name".to_string(), &"Some department".to_string());
        store
    }

    fn assert_log_entries(expected_number_of_errors: usize) {
        let assert_level = |l: &CapturedLog| assert_eq!(l.level, Debug, "Logging not at expected level");
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), expected_number_of_errors + 1, "Did not get expected number of log entries");
            assert_eq!(captured_logs[0].body, format!("Checking for command matching text \"{}\"", COMMAND));
            assert_level(&captured_logs[0]);
            for entry in &captured_logs[1..] {
                assert_eq!(entry.body, EXECUTOR_ERROR_MESSAGE, "Error log message not as expected");
                assert_level(entry);
            };
        });
    }

    fn success_stub_one(command: &str, employee_store: &mut EmployeeStoreImpl) -> Result<&'static str, &'static str> {
        assert_eq!((command, employee_store), (COMMAND, &mut mock_employee_store()));
        Ok(EXECUTOR_SUCCESS_MESSAGE_ONE)
    }

    fn success_stub_two(command: &str, employee_store: &mut EmployeeStoreImpl) -> Result<&'static str, &'static str> {
        assert_eq!((command, employee_store), (COMMAND, &mut mock_employee_store()));
        Ok(EXECUTOR_SUCCESS_MESSAGE_TWO)
    }

    fn error_stub(command: &str, employee_store: &mut EmployeeStoreImpl) -> Result<&'static str, &'static str> {
        assert_eq!((command, employee_store), (COMMAND, &mut mock_employee_store()));
        Err(EXECUTOR_ERROR_MESSAGE)
    }

    #[test]
    fn test_calls_one_executor_with_matching_command() {
        testing_logger::setup();
        let mut dispatcher = Dispatcher {
            command_executors: vec![success_stub_one],
            employee_store: mock_employee_store(),
        };
        assert_eq!(dispatcher.process(COMMAND), Ok(EXECUTOR_SUCCESS_MESSAGE_ONE));
        assert_log_entries(0);
    }

    #[test]
    fn test_bypasses_non_matching_executor() {
        testing_logger::setup();
        let mut dispatcher = Dispatcher {
            command_executors: vec![error_stub, success_stub_one],
            employee_store: mock_employee_store(),
        };
        assert_eq!(dispatcher.process(COMMAND), Ok(EXECUTOR_SUCCESS_MESSAGE_ONE));
        assert_log_entries(1);
    }

    #[test]
    fn test_stops_on_first_matching_executor() {
        testing_logger::setup();
        let mut dispatcher = Dispatcher {
            command_executors: vec![success_stub_one, success_stub_two, error_stub],
            employee_store: mock_employee_store(),
        };
        assert_eq!(dispatcher.process(COMMAND), Ok(EXECUTOR_SUCCESS_MESSAGE_ONE));
        assert_log_entries(0);
    }

    #[test]
    fn test_returns_error_if_no_matching_executors() {
        testing_logger::setup();
        let mut dispatcher = Dispatcher {
            command_executors: vec![error_stub, error_stub],
            employee_store: mock_employee_store(),
        };
        assert_eq!(dispatcher.process(COMMAND), Err(NO_MATCHES_ERROR_MESSAGE));
        assert_log_entries(2);
    }
}
