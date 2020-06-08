use crate::exercises::employee_management::employee_store::EmployeeStore;
use super::CommandExecutor;

static NON_MATCH_ERROR: Result<(), &str> = Err("Text command did not match pattern to retrieve all departments");

pub struct RetrieveAllCommandExecutor { }

impl CommandExecutor for RetrieveAllCommandExecutor {
    fn try_execute<T: 'static + EmployeeStore>(&self, command: &str, employee_store: &mut T)
        -> Result<(), &'static str>
    {
        if command == "Retrieve all departments" {
            info!("Retrieved full employee list:\n{:?}", employee_store.retrieve_all_employees());
            Ok(())
        } else {
            NON_MATCH_ERROR
        }
    }
}

#[cfg(test)]
mod tests {
    use log::Level;

    use crate::exercises::employee_management::employee_store;
    use crate::exercises::employee_management::employee_store::DepartmentInfo;

    use super::{CommandExecutor, RetrieveAllCommandExecutor};
    use super::NON_MATCH_ERROR;

    fn get_mock_return() -> Vec<DepartmentInfo> {
        vec![DepartmentInfo{ department: "Pie Analysis".to_string(), employee_names: vec![] }]
    }

    #[test]
    fn test_retrieve_all_command_ok_with_valid() {
        testing_logger::setup();

        let mut mock_store = employee_store::setup_mock(
            |mock| {
                mock
                    .expect_retrieve_all_employees()
                    .times(1)
                    .with()
                    .return_const(get_mock_return());
        });

        let executor = RetrieveAllCommandExecutor { };
        let result = executor.try_execute(
            &"Retrieve all departments".to_string(), &mut mock_store
        );

        assert_eq!(result, Ok(()));

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, format!("Retrieved full employee list:\n{:?}", get_mock_return()));
            assert_eq!(captured_logs[0].level, Level::Info);
        }
        )
    }

    #[test]
    fn test_retrieve_all_err_and_uncalled_with_invalid() {
        let mut mock_store = employee_store::setup_mock(
            |mock| {
                mock
                    .expect_retrieve_all_employees()
                    .times(0)
                    .with()
                    .return_const(get_mock_return());
            });

        let executor = RetrieveAllCommandExecutor {  };
        let result = executor.try_execute(
            &"This is not a retrieve command!".to_string(), &mut mock_store
        );

        assert_eq!(result, NON_MATCH_ERROR);
    }
}