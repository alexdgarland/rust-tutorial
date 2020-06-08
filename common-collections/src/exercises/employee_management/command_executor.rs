mod add_employee;
mod retrieve_all;
mod retrieve_department;

pub use add_employee::AddEmployeeCommandExecutor;
pub use retrieve_all::RetrieveAllCommandExecutor;
pub use retrieve_department::RetrieveDepartmentCommandExecutor;

use mockall_derive::automock;
use crate::exercises::employee_management::employee_store::EmployeeStore;

#[automock]
pub trait CommandExecutor {
    fn try_execute<T: 'static + EmployeeStore>(&self, command: &str, employee_store: &mut T)
        -> Result<(), &'static str>;
}
