mod add_employee;
mod retrieve_all;
mod retrieve_department;

pub use add_employee::AddEmployeeTextCommandExecutor;
pub use retrieve_all::RetrieveAllTextCommandExecutor;
pub use retrieve_department::RetrieveDepartmentTextCommandExecutor;

use mockall_derive::automock;

#[automock]
pub trait TextCommandExecutor {
    fn try_execute(&mut self, command: &str) -> Result<(), &'static str>;
}
