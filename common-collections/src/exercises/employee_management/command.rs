use std::result::Result;

use mockall_derive::automock;

use dispatcher::CommandDispatcher;
use handler::add_employee;
use handler::delete_employee;
use handler::list_departments;
use handler::retrieve_all_employees;
use handler::retrieve_employees_by_department;

use crate::exercises::employee_management::command::handler::CommandHandler;
use crate::exercises::employee_management::employee_store::{EmployeeStore, EmployeeStoreImpl};

mod handler;
pub(crate) mod dispatcher;

#[automock]
pub trait HandleCommand<E: 'static + EmployeeStore> {
    fn matches_command_text(&self, command_text: &str) -> bool;
    fn execute_command(&self, command_text: &str, employee_store: &mut E) -> Result<(), &'static str>;
    fn describe(&self) -> String;
}

pub type ConcreteDispatcher = CommandDispatcher<EmployeeStoreImpl, CommandHandler<EmployeeStoreImpl>>;

pub fn get_command_dispatcher() -> ConcreteDispatcher {
    let command_handlers = vec![
        add_employee::get_handler(),
        delete_employee::get_handler(),
        list_departments::get_handler(),
        retrieve_all_employees::get_handler(),
        retrieve_employees_by_department::get_handler()
    ];

    let employee_store = EmployeeStoreImpl::new();

    dispatcher::create_dispatcher(command_handlers, employee_store)
}
