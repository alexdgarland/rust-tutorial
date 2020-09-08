use employee_management;
use std::process;

fn main() {

    simple_logger::init().unwrap();

    let mut dispatcher = employee_management::get_command_dispatcher();

    if let Err(msg) = employee_management::run(&mut dispatcher) {
        eprintln!("Execution failed with error: \"{}\"", msg);
        process::exit(1);
    };

}
