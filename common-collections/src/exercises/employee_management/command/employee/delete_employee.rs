use regex::Regex;
use lazy_static::lazy_static;

use crate::exercises::employee_management::employee_store::EmployeeStore;
use crate::exercises::employee_management::employee_store::EmployeeDeletionResult::{
    SuccessfullyDeleted, NoSuchDepartment, EmployeeNotInDepartment,
};

use super::{parse_employee_command, EmployeeCommandParameters};

static NON_MATCH_ERROR: Result<(), &str> = Err("Text command did not match pattern to delete employee");

lazy_static! {
    static ref DELETE_EMPLOYEE_REGEX: Regex =
        Regex::new(r"^Delete (?P<employee_name>.*) from (?P<department>.*)$").unwrap();
}

pub fn delete_employee<E: EmployeeStore>(command: &str, employee_store: &mut E) -> Result<(), &'static str>
{
    match parse_employee_command(command, &*DELETE_EMPLOYEE_REGEX) {
        None =>
            NON_MATCH_ERROR,
        Some(params) => {
            let EmployeeCommandParameters { employee_name, department } = params;
            info!("Deleting employee \"{}\" from department \"{}\"", employee_name, department);
            match employee_store.delete_employee(&employee_name, &department) {
                SuccessfullyDeleted =>
                    info!("Successfully deleted employee \"{}\" from department \"{}\"", employee_name, department),
                NoSuchDepartment =>
                    info!("Department \"{}\" does not exist", department),
                EmployeeNotInDepartment =>
                    info!("Employee \"{}\" does not exist in department \"{}\"", employee_name, department),
            }
            Ok(())
        }
    }
}


#[cfg(test)]
mod tests {
    use log::Level;
    use mockall::predicate::eq;

    use super::{delete_employee, NON_MATCH_ERROR};
    use super::super::super::super::employee_store::MockEmployeeStore;
    use crate::exercises::employee_management::employee_store::EmployeeDeletionResult::{
        SuccessfullyDeleted, NoSuchDepartment, EmployeeNotInDepartment,
    };
    use crate::exercises::employee_management::employee_store::EmployeeDeletionResult;

    fn test_helper_handle_ok_execution(
        mock_store_deletion_result: EmployeeDeletionResult,
        expected_post_result_log_message: &str,
    ) {
        testing_logger::setup();

        let mut mock_store = MockEmployeeStore::new();

        mock_store.expect_delete_employee()
            .times(1)
            .with(
                eq(String::from("Bob Bobertson")),
                eq(String::from("Pie Quality Control")),
            ).return_const(mock_store_deletion_result);

        assert_eq!(
            delete_employee("Delete Bob Bobertson from Pie Quality Control", &mut mock_store),
            Ok(())
        );

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 2);
            assert_eq!(captured_logs[0].body,
                       "Deleting employee \"Bob Bobertson\" from department \"Pie Quality Control\""
            );
            assert_eq!(captured_logs[0].level, Level::Info);
            assert_eq!(captured_logs[1].body, expected_post_result_log_message);
            assert_eq!(captured_logs[1].level, Level::Info);
        });
    }

    #[test]
    fn test_delete_command_ok_with_valid_command_everything_exists() {
        test_helper_handle_ok_execution(
            SuccessfullyDeleted,
            "Successfully deleted employee \"Bob Bobertson\" from department \"Pie Quality Control\"",
        )
    }

    #[test]
    fn test_delete_command_ok_with_valid_command_department_doesnt_exist() {
        test_helper_handle_ok_execution(
            NoSuchDepartment,
            "Department \"Pie Quality Control\" does not exist",
        )
    }

    #[test]
    fn test_delete_command_ok_with_valid_command_employee_doesnt_exist() {
        test_helper_handle_ok_execution(
            EmployeeNotInDepartment,
            "Employee \"Bob Bobertson\" does not exist in department \"Pie Quality Control\"",
        )
    }

    #[test]
    fn test_delete_command_err_and_uncalled_with_invalid() {
        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_delete_employee()
            .times(0)
            .return_const(SuccessfullyDeleted);
        assert_eq!(
            delete_employee("This is not an add command!", &mut mock_store),
            NON_MATCH_ERROR
        );
    }
}
