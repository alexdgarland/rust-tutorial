use crate::exercises::employee_management::employee_store::EmployeeStore;
use super::TextCommandExecutor;

struct RetrieveDepartmentTextCommandExecutor<'a> {
    employee_store: &'a mut Box<dyn EmployeeStore>
}

impl TextCommandExecutor for RetrieveDepartmentTextCommandExecutor<'_> {
    fn try_execute(&mut self, command: &String) -> Result<(), String> {
        // TODO - implement this properly (with tests)
        Ok(())
    }
}
