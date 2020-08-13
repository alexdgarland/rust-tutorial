use std::fmt::Debug;

use mockall_derive::automock;

pub use implementation::EmployeeStoreImpl;

mod implementation;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct DepartmentInfo {
    pub department: String,
    pub employee_names: Vec<String>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum EmployeeDeletionResult {
    SuccessfullyDeleted,
    NoSuchDepartment,
    EmployeeNotInDepartment,
}

#[automock]
pub trait EmployeeStore {
    fn add_employee(&mut self, employee_name: &String, department: &String);

    fn retrieve_employees_by_department(&self, department: &String) -> Option<Vec<String>>;

    fn retrieve_all_employees(&self) -> Vec<DepartmentInfo>;

    fn list_departments(&self) -> Vec<String>;

    fn delete_department(&mut self, department: &String) -> Result<DepartmentInfo, String>;

    fn delete_employee(&mut self, employee_name: &String, department: &String) -> EmployeeDeletionResult;
}
