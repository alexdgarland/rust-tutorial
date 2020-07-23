use crate::exercises::employee_management::employee_store::EmployeeStore;

static NON_MATCH_ERROR: Result<(), &str> = Err("Text command did not match pattern to list departments");

pub fn list_departments<E: EmployeeStore>(command: &str, employee_store: &mut E) -> Result<(), &'static str>
{
    if command == "List departments" {
        info!("Retrieving department list");
        info!("{:?}", employee_store.list_departments());
        Ok(())
    } else {
        NON_MATCH_ERROR
    }
}

#[cfg(test)]
mod tests {
    use log::Level::Info;

    use super::{list_departments, NON_MATCH_ERROR};
    use super::super::super::employee_store::MockEmployeeStore;

    fn get_mock_return() -> Vec<String> {
        vec!["Department A".to_string(), "Department B".to_string()]
    }

    #[test]
    fn test_list_departments_command_ok_with_valid() {
        testing_logger::setup();

        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_list_departments()
            .times(1)
            .with()
            .return_const(get_mock_return());

        assert_eq!(
            list_departments(&"List departments", &mut mock_store),
            Ok(())
        );

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 2);
            assert_eq!(captured_logs[0].body, "Retrieving department list");
            assert_eq!(captured_logs[0].level, Info);
            assert_eq!(captured_logs[1].body, format!("{:?}", get_mock_return()));
            assert_eq!(captured_logs[1].level, Info);
        });
    }

    #[test]
    fn test_list_departments_err_and_uncalled_with_invalid() {
        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_list_departments()
            .times(0)
            .with()
            .return_const(get_mock_return());
        assert_eq!(
            list_departments(&"This is not a list command!".to_string(), &mut mock_store),
            NON_MATCH_ERROR
        );
    }
}