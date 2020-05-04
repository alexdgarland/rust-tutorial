use std::collections::HashMap;

fn log(message: &str) {
    println!("\n***** DISPATCH - {} *****\n", message);
}

fn log_string(message: String) { log(&message[..]); }

struct Method {
    name: &'static str,
    callable: fn() -> (),
}

impl Method {
    fn run(&self) {
        log_string(format!("Running method \"{}\"", self.name));
        (self.callable)();
    }
}

pub struct DispatchMap {
    map: HashMap<&'static str, Method>
}

impl DispatchMap {

    pub fn create(method_details: Vec<(&'static str, fn() -> ())>) -> DispatchMap {
        let mut map: HashMap<&str, Method> = HashMap::new();
        for (name, callable) in method_details {
            map.insert(name, Method { name, callable });
        }
        DispatchMap { map }
    }

    fn run_named_method(&self, method_name: &String) {
        match self.map.get(&method_name[..]) {
            Some(method) => {
                method.run();
            }
            None => {
                let message = format!(
                    "Method {} not found, valid methods are {:?}", method_name, self.map.keys()
                );
                log_string(message);
            }
        };
    }

    fn run_all(&self) {
        log("Running all available methods");
        for (_, method) in self.map.iter() {
            method.run();
        }
    }

    pub fn execute(&self, method_name: &Option<&String>) {
        match method_name {
            None => {
                log("No method name provided");
                self.run_all();
            }
            Some(method_name) => {
                self.run_named_method(method_name);
            }
        }
    }
}
