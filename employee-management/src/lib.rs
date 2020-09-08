#[macro_use]
extern crate log;
extern crate simple_logger;

mod command;
mod employee_store;

use std::io;
use command::{HandleCommand, CommandDispatcher};
use employee_store::EmployeeStore;

pub use command::get_command_dispatcher;

fn show_usage<E: 'static + EmployeeStore, H: HandleCommand<E>>(dispatcher: &CommandDispatcher<E, H>) {
    info!("Showing usage");
    println!("\n{}", dispatcher.get_usage_text());
    println!("Alternatively, enter:");
    println!(" - \"Help\" to show this usage info");
    println!(" - \"Quit\" to exit the program\n");
}

fn get_string(message: &str) -> io::Result<String> {
    println!("{}", message);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

pub fn run<E: 'static + EmployeeStore, H: HandleCommand<E>>(dispatcher: &mut CommandDispatcher<E, H>) -> Result<(), String> {

    show_usage(&dispatcher);

    loop {
        match get_string("Please enter a text command:") {
            Ok(raw_string) => {

                let text_command: &str = &raw_string.trim()[..];

                if text_command == "Quit" {
                    return Ok(());
                }
                if text_command == "Help" {
                    show_usage(&dispatcher);
                }
                else {
                    match dispatcher.process_command(text_command) {
                        Ok(msg) =>
                            debug!("Command \"{}\" processed okay - {}", text_command, msg),
                        Err(msg) => {
                            error!("{}", msg);
                            error!("Error processing command \"{}\", please try again", text_command);
                        }
                    };
                }
            }
            Err(e) => {
                return Err(format!("There was an error reading stdin: {:?}", e));
            }
        }
    }

}
