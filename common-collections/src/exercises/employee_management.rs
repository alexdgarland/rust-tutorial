use std::io;

use command_dispatcher::Dispatcher;

use crate::exercises::employee_management::employee_store::EmployeeStore;

mod employee_store;
// TODO - might make sense to restructure this as command::Dispatcher and command::executor::* or similar
mod command_dispatcher;
mod command_executor;

// TODO - maybe add some additional methods - list departments (without employees), delete employees/ departments?
// TODO - add help/ usage functionality

fn get_string(message: &str) -> io::Result<String> {
    println!("{}", message);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn process_command<E: EmployeeStore>(text_command: &str, dispatcher: &mut Dispatcher<E, ()>) {
    match dispatcher.process(text_command) {
        Err(message) => {
            error!("{}", message);
            error!("Error processing command \"{}\", please try again", text_command)
        },
        _ =>
            debug!("Command \"{}\" processed okay", text_command)
    }
}

pub fn demo_employee_management() {

    let mut dispatcher = command_dispatcher::create_dispatcher();

    loop {
        match get_string("Please enter a text command:") {
            Ok(raw_string) => {
                let text_command: &str = &raw_string.trim()[..];
                if text_command == "Quit" {
                    break;
                }
                process_command(text_command, &mut dispatcher);
            }
            Err(e) =>
                error!("There was an error reading stdin: {:?}", e)
        }
    }

}
