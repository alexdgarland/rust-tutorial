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

    fn help(&self) {
        println!("Common Collections Tutorial Code\n");
        println!("Valid methods are: {:?}\n", self.map.keys());
        println!(" - Call with one arg to run a specific named method");
        println!(" - Call without args to run all available methods");
        println!(" - Call with one arg \"help\" to show this usage info\n");
    }

    fn run_named_method(&self, method_name: &String) {
        if method_name == "help" {
            log("Showing help");
            self.help();
        }
        else {
            match self.map.get(&method_name[..]) {
                Some(method) => {
                    method.run();
                }
                None => {
                    log_string(format!("Method \"{}\" not found", method_name));
                    self.help();
                }
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
