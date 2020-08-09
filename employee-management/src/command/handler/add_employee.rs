use super::{ParsedArgMap, CommandHandler, CommandExecutor};
use crate::employee_store::EmployeeStore;
use regex::Regex;

const MATCH_PATTERN_DESCRIPTION: &'static str = "Add (employee name) to (department name)";
const REGEX_PATTERN: &'static str = r"^Add (?P<employee_name>.*) to (?P<department>.*)$";

pub fn get_handler<E: EmployeeStore>() -> CommandHandler<E> {
    let executor: CommandExecutor<E> = |arg_map: ParsedArgMap, store: &mut E| {
        let employee_name = arg_map.get("employee_name").unwrap();
        let department = arg_map.get("department").unwrap();
        match store
            .retrieve_employees_by_department(department)
            .map(|employees| employees.iter().any(|e| e == employee_name))
        {
            // This check could in some cases be handled as (e.g.) a unique constraint on underlying data store,
            // but let's assume we want to do this as a business logic concern in this layer independent of storage impl
            Some(true) => {
                error!("Employee \"{}\" already exists in department \"{}\" and cannot be added", employee_name, department);
                Err("Employee already exists in department")
            },
            _ => {
                store.add_employee(employee_name, department);
                info!("Successfully added employee \"{}\" to department \"{}\"", employee_name, department);
                Ok(())
            }
        }
    };

    CommandHandler::new(
        MATCH_PATTERN_DESCRIPTION,
        Regex::new(REGEX_PATTERN).unwrap(),
        vec!["employee_name", "department"],
        executor,
    )
}


#[cfg(test)]
mod tests {
    use super::get_handler;
    use crate::command::HandleCommand;
    use crate::command::handler::CommandHandler;
    use mockall::predicate::eq;
    use crate::employee_store::MockEmployeeStore;
    use log::Level;

    const MATCHING_COMMAND: &str = "Add Bob Bobertson to Pie Quality Control";
    const NON_MATCHING_COMMAND: &'static str = "Add Bob Bobertson into the Pie Eating department";

    fn run_test_against_matcher(command_text: &str, expected_return: bool) {
        let test_handler: CommandHandler<MockEmployeeStore> = get_handler();
        assert_eq!(test_handler.matches_command_text(command_text), expected_return)
    }

    #[test]
    fn test_matcher_handles_matching_pattern() {
        run_test_against_matcher(MATCHING_COMMAND, true);
    }

    #[test]
    fn test_matcher_handles_non_matching_pattern() {
        run_test_against_matcher(NON_MATCHING_COMMAND, false);
    }

    #[test]
    fn test_executor_calls_expected_method_on_store() {
        testing_logger::setup();

        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_retrieve_employees_by_department()
            .times(1)
            .with(eq(String::from("Pie Quality Control")))
            .return_const(None);
        mock_store
            .expect_add_employee()
            .times(1)
            .with(
                eq("Bob Bobertson".to_string()),
                eq("Pie Quality Control".to_string()),
            ).return_const(());

        let result = get_handler()
            .execute_command(MATCHING_COMMAND, &mut mock_store);

        assert_eq!(result, Ok(()));

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body,
                       "Successfully added employee \"Bob Bobertson\" to department \"Pie Quality Control\"");
            assert_eq!(captured_logs[0].level, Level::Info);
        })
    }

    #[test]
    fn test_executor_errors_without_adding_if_existing_employee_added_to_same_department() {
        testing_logger::setup();

        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_retrieve_employees_by_department()
            .times(1)
            .with(eq("Pie Quality Control".to_string()))
            .return_const(Some(vec!["Bob Bobertson".to_string()]));
        mock_store
            .expect_add_employee()
            .times(0);

        let result = get_handler()
            .execute_command(MATCHING_COMMAND, &mut mock_store);

        assert_eq!(result, Err("Employee already exists in department"));

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(
                captured_logs[0].body,
                "Employee \"Bob Bobertson\" already exists in department \"Pie Quality Control\" and cannot be added"
            );
            assert_eq!(captured_logs[0].level, Level::Error);
        })
    }
}