use std::result::Result;

use mockall_derive::automock;

use dispatcher::CommandDispatcher;
use handler::CommandHandler;
use crate::employee_store::{EmployeeStore, EmployeeStoreImpl};
pub use dispatcher::CommandProcessingResult;

mod handler;
mod dispatcher;

#[automock]
pub trait HandleCommand<E: 'static + EmployeeStore> {
    fn matches_command_text(&self, command_text: &str) -> bool;
    fn execute_command(&self, command_text: &str, employee_store: &mut E) -> Result<(), &'static str>;
    fn describe(&self) -> String;
}

pub type ConcreteDispatcher = CommandDispatcher<EmployeeStoreImpl, CommandHandler<EmployeeStoreImpl>>;

pub fn get_command_dispatcher() -> ConcreteDispatcher {
    let command_handlers = handler::get_all_handlers();
    let employee_store = EmployeeStoreImpl::new();
    dispatcher::create_dispatcher(command_handlers, employee_store)
}
