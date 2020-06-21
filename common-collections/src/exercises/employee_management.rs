use std::io;

use command::Dispatcher;

use crate::exercises::employee_management::employee_store::EmployeeStore;

mod employee_store;
mod command;

// TODO - maybe add some additional methods - delete employees/ departments?

// TODO - it's easy to forget to add an entry here when adding new methods,
//  is there an easy way to combine this with the function impl?
//  In some languages I would make a class/ struct with a string and a method,
//  but in this case not sure if the approach to generics will cause issues - need to check
fn show_usage() {
    info!("Showing usage");
    println!("Employee Management - valid command formats:");
    println!(" - \"Add (employee name) to (department name)\"");
    println!(" - \"List departments\"");
    println!(" - \"Retrieve all departments\"");
    println!(" - \"Retrieve department (department name)\"");
    println!(" - \"Help\" to show this usage info");
    println!(" - \"Quit\" to exit the demo program");
}

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
            error!("Error processing command \"{}\", please try again", text_command);
            show_usage();
        },
        _ =>
            debug!("Command \"{}\" processed okay", text_command)
    }
}

pub fn demo_employee_management() {

    let mut dispatcher = command::create_dispatcher();

    show_usage();

    loop {
        match get_string("Please enter a text command:") {
            Ok(raw_string) => {
                let text_command: &str = &raw_string.trim()[..];
                if text_command == "Quit" {
                    break;
                }
                if text_command == "Help" {
                    show_usage();
                }
                else {
                    process_command(text_command, &mut dispatcher);
                }
            }
            Err(e) =>
                error!("There was an error reading stdin: {:?}", e)
        }
    }

}
