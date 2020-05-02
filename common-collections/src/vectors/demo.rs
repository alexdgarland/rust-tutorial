use super::retrieve;
use super::create;
use super::modify;
use super::spreadsheet;

struct DemoExample {
    description: &'static str,
    function: fn() -> (),
}

impl DemoExample {
    fn run(&self) {
        println!("**** Demoing {} ****\n", self.description);
        (self.function)();
        println!();
    }
}

pub fn demo_vectors() {
    let examples = [
        DemoExample {
            description: "creation of vectors",
            function: create::show_created_vectors,
        },
        DemoExample {
            description: "retrieval of vector elements",
            function: retrieve::read_vector_elements,
        },
        DemoExample {
            description: "modification of vectors",
            function: modify::show_modified_vectors
        },
        DemoExample {
            description: "using vector of enums to model a spreadsheet",
            function: spreadsheet::show_spreadsheet_operations
        }
    ];

    for example in examples.iter() {
        example.run();
    }
}
