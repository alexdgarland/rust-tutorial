mod add_employee;
mod retrieve_all;
mod retrieve_department;

pub use add_employee::AddEmployeeTextCommandExecutor;
pub use retrieve_all::RetrieveAllTextCommandExecutor;
pub use retrieve_department::RetrieveDepartmentTextCommandExecutor;

pub trait TextCommandExecutor {
    fn try_execute(&mut self, command: &String) -> Result<(), &str>;
}
