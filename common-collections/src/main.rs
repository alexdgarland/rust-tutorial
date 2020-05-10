use std::env;

mod vectors;
mod strings;
mod dispatch;
mod exercises;

use dispatch::Dispatcher;

fn main() {
    let dispatcher = Dispatcher::create(
        vec![
            ("vectors", vectors::demo_vectors),
            ("strings", strings::demo_strings),
            ("intlist", exercises::demo_int_list),
            ("piglatin", exercises::demo_pig_latin)
        ]
    );

    let args: Vec<String> = env::args().collect();
    let method_name = args.get(1);

    dispatcher.execute(&method_name);
}
