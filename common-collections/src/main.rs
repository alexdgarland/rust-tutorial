use std::env;

mod vectors;
mod strings;
mod dispatch;
mod exercises;

use dispatch::DispatchMap;

fn main() {
    let dispatch_map = DispatchMap::create(
        vec![
            ("vectors", vectors::demo_vectors),
            ("strings", strings::demo_strings)
        ]
    );

    let args: Vec<String> = env::args().collect();
    let method_name = args.get(1);

    dispatch_map.execute(&method_name);
}
