mod employee_store;
mod text_command_dispatcher;
mod text_command_executor;

use text_command_executor::AddEmployeeTextCommandExecutor;
use crate::exercises::employee_management::text_command_executor::TextCommandExecutor;

pub fn demo_employee_management() {

    // TODO - this needs to be rewritten once dispatcher is properly implemented and can be run interactively in a loop

    let mut store = employee_store::create_employee_store();

    let mut add_executor = AddEmployeeTextCommandExecutor {
        employee_store: &mut store
    };

    let mut add = |cmd: &str| {
        info!("Trying to add employee using command \"{}\"", cmd);
        match add_executor.try_execute(&cmd.to_string()) {
            Err(message) =>
                error!("Error occured while trying to process add command - \"{}\"", message),
            Ok(_) =>
                info!("That worked fine :-)")
        };
    };

    add("Add Bob Bobertson to Pie Quality Control");
    add("Add SOMETHING SOMEWHERE");

    info!("Store now looks like: [ {:?} ]", store);
}
