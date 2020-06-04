use regex::Regex;

use lazy_static::lazy_static;

use crate::exercises::employee_management::employee_store::EmployeeStore;

use super::TextCommandExecutor;

static NON_MATCH_ERROR: Result<(), &str> = Err("Text command did not match pattern to retrieve employees by department");

pub struct RetrieveDepartmentTextCommandExecutor<'a> {
    pub employee_store: &'a mut Box<dyn EmployeeStore>
}

impl TextCommandExecutor for RetrieveDepartmentTextCommandExecutor<'_> {
    fn try_execute(&mut self, command: &str) -> Result<(), &'static str> {
        lazy_static! {
            static ref RETRIEVE_DEPARTMENT_REGEX: Regex =
                Regex::new(r"^Retrieve department (?P<department>.*)$").unwrap();
        }

        match RETRIEVE_DEPARTMENT_REGEX
            .captures(&command[..])
            .and_then(|captures| captures.name("department").map(|m| m.as_str().to_string()))
        {
            Some(department) => {
                info!("Retrieving employees for department {}", department);
                match self.employee_store.retrieve_employees_by_department(&department) {
                    Some(employees) =>
                        info!("{:?}", employees),
                    None =>
                        warn!("Department {} does not exist", department)
                }
                Ok(())
            }
            None =>
                NON_MATCH_ERROR
        }

    }
}

#[cfg(test)]
mod tests {
    use log::Level;
    use mockall::predicate::eq;

    use crate::exercises::employee_management::employee_store;
    use crate::exercises::employee_management::text_command_executor::retrieve_department::NON_MATCH_ERROR;

    use super::RetrieveDepartmentTextCommandExecutor;
    use super::super::TextCommandExecutor;

    fn get_mock_return() -> Vec<String> {
        vec!["Bob Bobertson".to_string(), "Weebl Bull".to_string()]
    }

    #[test]
    fn test_retrieve_department_command_ok_with_valid_department_exists() {
        testing_logger::setup();

        let mut mock_store = employee_store::setup_mock(
            |mock| {
                mock
                    .expect_retrieve_employees_by_department()
                    .times(1)
                    .with(eq("Pie Quality Control".to_string()))
                    .return_const(Some(get_mock_return()));
            });

        let mut executor = RetrieveDepartmentTextCommandExecutor { employee_store: &mut mock_store };

        let command = String::from("Retrieve department Pie Quality Control");

        assert_eq!(executor.try_execute(&command), Ok(()));

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 2);
            assert_eq!(captured_logs[0].body, "Retrieving employees for department Pie Quality Control");
            assert_eq!(captured_logs[0].level, Level::Info);
            assert_eq!(captured_logs[1].body, format!("{:?}", get_mock_return()));
            assert_eq!(captured_logs[1].level, Level::Info);
        });
    }

    #[test]
    fn test_retrieve_department_command_ok_with_valid_department_doesnt_exist() {
        testing_logger::setup();

        let mut mock_store = employee_store::setup_mock(
            |mock| {
                mock
                    .expect_retrieve_employees_by_department()
                    .times(1)
                    .with(eq("Pie Quality Control".to_string()))
                    .return_const(None);
            });

        let mut executor = RetrieveDepartmentTextCommandExecutor { employee_store: &mut mock_store };

        let command = String::from("Retrieve department Pie Quality Control");

        assert_eq!(executor.try_execute(&command), Ok(()));

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 2);
            assert_eq!(captured_logs[0].body, "Retrieving employees for department Pie Quality Control");
            assert_eq!(captured_logs[0].level, Level::Info);
            assert_eq!(captured_logs[1].body, format!("Department Pie Quality Control does not exist"));
            assert_eq!(captured_logs[1].level, Level::Warn);
        });
    }

    #[test]
    fn test_retrieve_department_command_err_and_uncalled_with_invalid() {
        let mut mock_store = employee_store::setup_mock(
            |mock| {
                mock
                    .expect_retrieve_employees_by_department()
                    .times(0)
                    .return_const(Some(get_mock_return()));
            });

        let mut executor = RetrieveDepartmentTextCommandExecutor { employee_store: &mut mock_store };

        let command = String::from("This is not a command to retrieve by department!");

        assert_eq!(executor.try_execute(&command), NON_MATCH_ERROR);
    }
}