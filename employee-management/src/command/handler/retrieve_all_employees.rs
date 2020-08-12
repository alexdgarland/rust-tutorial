use super::{ParsedArgMap, CommandHandler, CommandExecutor};
use crate::employee_store::EmployeeStore;
use regex::Regex;

const MATCH_PATTERN_DESCRIPTION: &'static str = "Retrieve all departments";
const REGEX_PATTERN: &'static str = r"^Retrieve all departments$";

pub fn get_handler<E: EmployeeStore>() -> CommandHandler<E> {
    let executor: CommandExecutor<E> = |_arg_map: ParsedArgMap, store: &mut E| {
        info!("Retrieving full employee list");
        let departments = store.retrieve_all_employees();
        info!("Number of departments found: {}", departments.len());
        for dept_info in departments {
            info!("{} - {}", dept_info.department, dept_info.employee_names.join(", "));
        }
        Ok(())
    };

    CommandHandler::new(
        MATCH_PATTERN_DESCRIPTION,
        Regex::new(REGEX_PATTERN).unwrap(),
        vec![],
        executor,
    )
}


#[cfg(test)]
mod tests {
    use super::get_handler;
    use crate::command::HandleCommand;
    use crate::command::handler::CommandHandler;
    use crate::employee_store::{MockEmployeeStore, DepartmentInfo};
    use log::Level;

    const MATCHING_COMMAND: &str = "Retrieve all departments";
    const NON_MATCHING_COMMAND: &'static str = "Get me all the departments!";

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

        let mock_return_department_infos = vec![
            DepartmentInfo{
                department: "Pie Analysis".to_string(),
                employee_names: vec!["Bob Bobertson".to_string(), "Weebl Bull".to_string()]
            },
            DepartmentInfo{
                department: "Stealthy Buccaneering".to_string(),
                employee_names: vec!["Chris the Ninja Pirate".to_string()]
            }
        ];
        let mut mock_store = MockEmployeeStore::new();
        mock_store
            .expect_retrieve_all_employees()
            .times(1)
            .with()
            .return_once(move || mock_return_department_infos);

        let result = get_handler()
            .execute_command(MATCHING_COMMAND, &mut mock_store);

        assert_eq!(result, Ok(()));

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 4);
            for log in captured_logs {
                assert_eq!(log.level, Level::Info);
            }
            assert_eq!(captured_logs[0].body, "Retrieving full employee list");
            assert_eq!(captured_logs[1].body, "Number of departments found: 2");
            assert_eq!(captured_logs[2].body, "Pie Analysis - Bob Bobertson, Weebl Bull");
            assert_eq!(captured_logs[3].body, "Stealthy Buccaneering - Chris the Ninja Pirate");
        });
    }
}