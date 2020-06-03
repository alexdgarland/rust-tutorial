use crate::exercises::employee_management::employee_store::EmployeeStore;
use super::TextCommandExecutor;

struct RetrieveAllTextCommandExecutor<'a> {
    employee_store: &'a mut Box<dyn EmployeeStore>
}

impl TextCommandExecutor for RetrieveAllTextCommandExecutor<'_> {
    fn try_execute(&mut self, command: &String) -> Result<(), String> {
        if command == "Retrieve all departments" {
            info!("Retrieved full employee list:\n{:?}", self.employee_store.retrieve_all_employees());
            Ok(())
        } else {
            Err("Text command did not match pattern to retrieve all departments".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use log::Level;

    use crate::exercises::employee_management::employee_store;
    use crate::exercises::employee_management::employee_store::DepartmentInfo;

    use super::{TextCommandExecutor, RetrieveAllTextCommandExecutor};

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

        let mut executor = RetrieveAllTextCommandExecutor { employee_store: &mut mock_store };

        assert_eq!(executor.try_execute(&"Retrieve all departments".to_string()), Ok(()));

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

        let mut executor = RetrieveAllTextCommandExecutor { employee_store: &mut mock_store };

        assert_eq!(
            executor.try_execute(&"This is not a retrieve command!".to_string()),
            Err("Text command did not match pattern to retrieve all departments".to_string())
        );
    }
}