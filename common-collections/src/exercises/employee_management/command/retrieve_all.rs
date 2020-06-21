use crate::exercises::employee_management::employee_store::EmployeeStore;

static NON_MATCH_ERROR: Result<(), &str> = Err("Text command did not match pattern to retrieve all departments");

pub fn retrieve_all<E: EmployeeStore>(command: &str, employee_store: &mut E) -> Result<(), &'static str>
{
    if command == "Retrieve all departments" {
        info!("Retrieving full employee list");
        info!("{:?}", employee_store.retrieve_all_employees());
        Ok(())
    } else {
        NON_MATCH_ERROR
    }
}

#[cfg(test)]
mod tests {
    use log::Level::Info;

    use crate::exercises::employee_management::employee_store::DepartmentInfo;

    use super::{NON_MATCH_ERROR, retrieve_all};
    use super::super::shared_test_setup;

    fn get_mock_return() -> Vec<DepartmentInfo> {
        vec![DepartmentInfo{ department: "Pie Analysis".to_string(), employee_names: vec![] }]
    }

    #[test]
    fn test_retrieve_all_command_ok_with_valid() {
        testing_logger::setup();

        let mut mock_store = shared_test_setup::setup_store_mock(
            |mock| {
                mock
                    .expect_retrieve_all_employees()
                    .times(1)
                    .with()
                    .return_const(get_mock_return());
        });

        assert_eq!(
            retrieve_all(&"Retrieve all departments", &mut mock_store),
            Ok(())
        );

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 2);
            assert_eq!(captured_logs[0].body, "Retrieving full employee list");
            assert_eq!(captured_logs[0].level, Info);
            assert_eq!(captured_logs[1].body, format!("{:?}", get_mock_return()));
            assert_eq!(captured_logs[1].level, Info);
        });
    }

    #[test]
    fn test_retrieve_all_err_and_uncalled_with_invalid() {
        let mut mock_store = shared_test_setup::setup_store_mock(
            |mock| {
                mock
                    .expect_retrieve_all_employees()
                    .times(0)
                    .with()
                    .return_const(get_mock_return());
            });

        assert_eq!(
            retrieve_all(&"This is not a retrieve command!".to_string(), &mut mock_store),
            NON_MATCH_ERROR
        );
    }
}