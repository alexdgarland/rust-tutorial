use crate::exercises::employee_management::command_dispatcher::{Dispatcher, Executor};
use crate::exercises::employee_management::command_dispatcher::create_dispatcher;
use crate::exercises::employee_management::command_executor::*;
use crate::exercises::employee_management::employee_store::{EmployeeStore, EmployeeStoreImpl};

mod employee_store;
// TODO - might make sense to restructure this as command::Dispatcher and command::executor::* or similar
mod command_dispatcher;
mod command_executor;

pub fn demo_employee_management() {

    // TODO - wrap this in a loop that reads stdin to make it interactive

    let mut dispatcher = command_dispatcher::create_dispatcher(
        vec![add_employee, retrieve_all, retrieve_by_department]
    );

    dispatcher.process("Add Bob Bobertson to Pie Recipe Generation");
    dispatcher.process("Add Weebl Bull to Pie Consumption");
    dispatcher.process("Retrieve department Pie Recipe Generation");
    dispatcher.process("Retrieve department Pie Consumption");
    dispatcher.process("Retrieve all departments");

    // TODO - maybe add some additional methods - list departments (without employees), delete employees/ departments?
}
