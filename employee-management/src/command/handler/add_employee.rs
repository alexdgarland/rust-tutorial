use super::{ParsedArgMap, CommandHandler, CommandExecutor};
use crate::employee_store::EmployeeStore;
use regex::Regex;

const MATCH_PATTERN_DESCRIPTION: &'static str = "Add (employee name) to (department name)";
const REGEX_PATTERN: &'static str = r"^Add (?P<employee_name>.*) to (?P<department>.*)$";

pub fn get_handler<E: EmployeeStore>() -> CommandHandler<E> {
    // TODO - maybe return an error if employee with same name already exists?
    let executor: CommandExecutor<E> = |arg_map: ParsedArgMap, store: &mut E| {
        store.add_employee(
            arg_map.get("employee_name").unwrap(),
            arg_map.get("department").unwrap(),
        );
        Ok(())
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
        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_add_employee()
            .times(1)
            .with(
                eq(String::from("Bob Bobertson")),
                eq(String::from("Pie Quality Control")),
            ).return_const(());

        let result = get_handler()
            .execute_command(MATCHING_COMMAND, &mut mock_store);

        assert_eq!(result, Ok(()));
    }
}