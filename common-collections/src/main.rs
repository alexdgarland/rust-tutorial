use std::env;

mod vectors;
mod strings;
mod dispatch;

use dispatch::DispatchMap;

fn main() {
    let dispatch_map = DispatchMap::create(
        vec![
            ("vectors", vectors::demo_vectors),
            ("strings", strings::demo_strings)
        ]
    );

    let method_name = env::args().collect().get(1);

    dispatch_map.execute(&method_name);
}
