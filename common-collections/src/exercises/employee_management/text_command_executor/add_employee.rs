use regex::{Captures, Regex};
use lazy_static::lazy_static;

use crate::exercises::employee_management::employee_store::EmployeeStore;
use super::TextCommandExecutor;

static NON_MATCH_ERROR: Result<(), &str> = Err("Text command did not match pattern to add employee");

pub struct AddEmployeeTextCommandExecutor<'a> {
    pub employee_store: &'a mut Box<dyn EmployeeStore>
}

impl TextCommandExecutor for AddEmployeeTextCommandExecutor<'_> {
    fn try_execute(&mut self, command: &String) -> Result<(), &str> {
        lazy_static! {
            static ref ADD_EMPLOYEE_REGEX: Regex =
                Regex::new(r"^Add (?P<employee_name>.*) to (?P<department>.*)$").unwrap();
        }

        fn extract_fields(captures: Captures) -> Option<(Option<String>, Option<String>)> {
            let extract = |key: &str|
                captures
                    .name(key)
                    .map(|m| m.as_str().to_string());

            Some((extract("employee_name"), extract("department")))
        }

        match ADD_EMPLOYEE_REGEX.captures(&command[..]).and_then(extract_fields)
        {
            Some((Some(employee_name), Some(department))) => {
                info!("Adding employee {} to department {}", employee_name, department);
                self.employee_store.add_employee(&employee_name, &department);
                Ok(())
            }
            _ =>
                NON_MATCH_ERROR
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;
    use log::Level;

    use crate::exercises::employee_management::employee_store;
    use super::super::TextCommandExecutor;
    use super::AddEmployeeTextCommandExecutor;
    use crate::exercises::employee_management::text_command_executor::add_employee::NON_MATCH_ERROR;

    #[test]
    fn test_add_command_ok_with_valid() {
        testing_logger::setup();

        let mut mock_store = employee_store::setup_mock(
            |mock| {
                mock
                    .expect_add_employee()
                    .times(1)
                    .with(
                        eq(String::from("Bob Bobertson")),
                        eq(String::from("Pie Quality Control")),
                    ).return_const(());
            });

        let mut executor = AddEmployeeTextCommandExecutor { employee_store: &mut mock_store };

        let command = "Add Bob Bobertson to Pie Quality Control".to_string();

        assert_eq!(executor.try_execute(&command), Ok(()));

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, "Adding employee Bob Bobertson to department Pie Quality Control");
            assert_eq!(captured_logs[0].level, Level::Info);
        });
    }

    #[test]
    fn test_add_command_err_and_uncalled_with_invalid() {
        let mut mock_store = employee_store::setup_mock(
            |mock| {
                mock
                    .expect_add_employee()
                    .times(0)
                    .return_const(());
            });

        let mut executor = AddEmployeeTextCommandExecutor { employee_store: &mut mock_store };

        let command = String::from("This is not an add command!");

        assert_eq!(executor.try_execute(&command), NON_MATCH_ERROR);
    }
}
