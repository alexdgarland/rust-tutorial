use mockall_derive::automock;
use super::employee_store;
use super::employee_store::EmployeeStore;
use regex::{Regex, Captures};

#[automock]
trait StdOutPrinter {
    fn println(&self, s: String) -> ();
}

struct StdOutPrinterImpl { }

impl StdOutPrinter for StdOutPrinterImpl {
    fn println(&self, s: String) {
        println!("{}", s);
    }
}

struct TextCommandProcessor {
    employee_store: Box<dyn EmployeeStore>,
    printer: Box<dyn StdOutPrinter>
}

impl TextCommandProcessor {

    pub fn create() -> TextCommandProcessor {
        TextCommandProcessor {
            employee_store: employee_store::create_employee_store(),
            printer: Box::new(StdOutPrinterImpl {})
        }
    }

    // TODO - this should return a Result?
    pub fn process(&mut self, text_command: String) {
    // TODO This currently only handles the add_employee operation -
    // TODO need to implement handling of other methods (without too much code duplication).

        fn extract_fields(captures: Captures) -> Option<(Option<String>, Option<String>)> {
            let extract = |key: &str| {
                captures
                    .name(key)
                    .map(|m| String::from(m.as_str()))
            };
            Some((extract("employee_name"), extract("department")))
        };

        // TODO - wrap this in a lazy_static like https://rust-lang-nursery.github.io/rust-cookbook/text/regex.html
        let regex = Regex::new(r"^Add (?P<employee_name>.*) to (?P<department>.*)$").unwrap();

        let extracted= regex.captures(&text_command[..]).and_then(extract_fields);

        if let Some((Some(employee_name), Some(department))) = extracted
        {
            // self.printer.println(
            //     format!("Adding employee {} to department {}", employee_name, department)
            // );
            self.employee_store.add_employee(employee_name, department);
        };

    }

}

#[cfg(test)]
mod text_command_processor_tests {
    use mockall::predicate::eq;
    use super::MockStdOutPrinter;
    use super::super::employee_store::MockEmployeeStore;
    use super::TextCommandProcessor;

    #[test]
    fn test_processes_valid_add_command() {

        let mut mock_printer = MockStdOutPrinter::new();
        // TODO - work out why this expectation is not passing
        // mock_printer
        //     .expect_println()
        //     .times(1)
        //     .with(eq(String::from(
        //         "Adding employee Bob Bobertson to department Pie Quality Control"
        //     )));

        let mut mock_employee_store = MockEmployeeStore::new();
        mock_employee_store
            .expect_add_employee()
            .times(1)
            .with(
                eq(String::from("Bob Bobertson")),
                eq(String::from("Pie Quality Control"))
            )
            .return_const(());

        // TODO - Seems maybe a bit off that we're setting the whole object here as mut...
        // ...to be able to update data within the contained Employee Store?
        // See if there is a better way to do this.
        let mut processor = TextCommandProcessor {
            employee_store: Box::new(mock_employee_store),
            printer: Box::new(mock_printer)
        };

        let command = String::from("Add Bob Bobertson to Pie Quality Control");
        processor.process(command);
    }

    // TODO - implement these other tests
    // #[test]
    // fn test_processes_valid_retrieve_all_employees_command() {
    //     assert!(false);
    // }
    //
    // #[test]
    // fn test_processes_valid_retrieve_department_employees_command() {
    //     assert!(false);
    // }
    //
    // #[test]
    // fn test_processes_invalid_command_returning_error_result() {
    //     assert!(false);
    // }

}
