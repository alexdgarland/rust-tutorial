#[macro_use]
extern crate log;
extern crate simple_logger;

mod command;
mod employee_store;

use command::ConcreteDispatcher;
use command::CommandProcessingResult::{ NoMatchingHandlerFound, HandlerExecutionFailed, Success };
use std::io;

// TODO - maybe add some additional methods - delete departments?

fn show_usage(dispatcher: &ConcreteDispatcher) {
    info!("Showing usage");
    println!("{}", dispatcher.get_usage_text());
}

fn get_string(message: &str) -> io::Result<String> {
    println!("{}", message);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn process_command(text_command: &str, dispatcher: &mut ConcreteDispatcher) {
    match dispatcher.process_command(text_command) {
        NoMatchingHandlerFound => {
            error!("No match could be found to execute the submitted command \"{}\"", text_command);
            show_usage(dispatcher);
        },
        HandlerExecutionFailed(error_message) => {
            error!("{}", error_message);
            error!("Error processing command \"{}\", please try again", text_command);
            show_usage(dispatcher);
        }
        Success =>
            debug!("Command \"{}\" processed okay", text_command)
    }
}

fn main() {

    simple_logger::init().unwrap();

    let mut dispatcher = command::get_command_dispatcher();

    show_usage(&dispatcher);

    loop {
        match get_string("Please enter a text command:") {
            Ok(raw_string) => {
                let text_command: &str = &raw_string.trim()[..];
                if text_command == "Quit" {
                    break;
                }
                if text_command == "Help" {
                    show_usage(&dispatcher);
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
