use super::implementation as im;

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
            function: im::show_created_vectors,
        },
        DemoExample {
            description: "retrieval of vector elements",
            function: im::read_vector_elements,
        }
    ];

    for example in examples.iter() {
        example.run();
    }
}
