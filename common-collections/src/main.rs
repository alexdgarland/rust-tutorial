#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate testing_logger;

use std::env;

use dispatch::Dispatcher;

mod vectors;
mod strings;
mod dispatch;
mod exercises;

fn main() {
    simple_logger::init().unwrap();

    let dispatcher = Dispatcher::create(
        vec![
            ("vectors", vectors::demo_vectors),
            ("strings", strings::demo_strings),
            ("intlist", exercises::demo_int_list),
            ("piglatin", exercises::demo_pig_latin),
            ("employee", exercises::demo_employee_management),
        ]
    );

    let args: Vec<String> = env::args().collect();
    let method_name = args.get(1);

    dispatcher.execute(&method_name);
}
