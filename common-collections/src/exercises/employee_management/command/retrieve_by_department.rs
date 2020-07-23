use regex::Regex;

use lazy_static::lazy_static;

use crate::exercises::employee_management::employee_store::EmployeeStore;

static NON_MATCH_ERROR: Result<(), &str> = Err("Text command did not match pattern to retrieve employees by department");

pub fn retrieve_by_department<E: EmployeeStore>(command: &str, employee_store: &mut E) -> Result<(), &'static str>
{
    lazy_static! {
        static ref RETRIEVE_DEPARTMENT_REGEX: Regex =
            Regex::new(r"^Retrieve department (?P<department>.*)$").unwrap();
    }

    match RETRIEVE_DEPARTMENT_REGEX
        .captures(&command[..])
        .and_then(|captures| captures.name("department").map(|m| m.as_str().to_string()))
    {
        Some(department) => {
            info!("Retrieving employees for department \"{}\"", department);
            match employee_store.retrieve_employees_by_department(&department) {
                Some(employees) =>
                    info!("{:?}", employees),
                None =>
                    warn!("Department \"{}\" does not exist", department)
            }
            Ok(())
        }
        None =>
            NON_MATCH_ERROR
    }

}


#[cfg(test)]
mod tests {
    use log::Level;
    use mockall::predicate::eq;

    use super::{NON_MATCH_ERROR, retrieve_by_department};
    use super::super::super::employee_store::MockEmployeeStore;

    fn get_mock_return() -> Vec<String> {
        vec!["Bob Bobertson".to_string(), "Weebl Bull".to_string()]
    }

    #[test]
    fn test_retrieve_department_command_ok_with_valid_department_exists() {
        testing_logger::setup();

        let mut mock_store = MockEmployeeStore::new();
         mock_store
             .expect_retrieve_employees_by_department()
             .times(1)
             .with(eq("Pie Quality Control".to_string()))
             .return_const(Some(get_mock_return()));

        assert_eq!(
            retrieve_by_department("Retrieve department Pie Quality Control", &mut mock_store),
            Ok(())
        );

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 2);
            assert_eq!(captured_logs[0].body, "Retrieving employees for department \"Pie Quality Control\"");
            assert_eq!(captured_logs[0].level, Level::Info);
            assert_eq!(captured_logs[1].body, format!("{:?}", get_mock_return()));
            assert_eq!(captured_logs[1].level, Level::Info);
        });
    }

    #[test]
    fn test_retrieve_department_command_ok_with_valid_department_doesnt_exist() {
        testing_logger::setup();

        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_retrieve_employees_by_department()
            .times(1)
            .with(eq("Pie Quality Control".to_string()))
            .return_const(None);

        assert_eq!(
            retrieve_by_department("Retrieve department Pie Quality Control", &mut mock_store),
            Ok(())
        );

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 2);
            assert_eq!(captured_logs[0].body, "Retrieving employees for department \"Pie Quality Control\"");
            assert_eq!(captured_logs[0].level, Level::Info);
            assert_eq!(captured_logs[1].body, format!("Department \"Pie Quality Control\" does not exist"));
            assert_eq!(captured_logs[1].level, Level::Warn);
        });
    }

    #[test]
    fn test_retrieve_department_command_err_and_uncalled_with_invalid() {
        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_retrieve_employees_by_department()
            .times(0)
            .return_const(Some(get_mock_return()));
        assert_eq!(
            retrieve_by_department("This is a bad command!", &mut mock_store),
            NON_MATCH_ERROR
        );
    }
}