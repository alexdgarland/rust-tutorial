use std::panic;

fn create_vectors() -> [Vec<i32>; 3] {
    let v1: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    return [v1, v2, v3];
}

pub fn show_created_vectors() {
    for v in create_vectors().iter() {
        println!("{:?}", v);
    }
}

fn create_reader(vector: &Vec<i32>, retriever: fn(&Vec<i32>, usize) -> ()) -> Box<dyn Fn(usize) -> ()> {
    let vc = vector.clone();
    let reader = move |index: usize| retriever(&vc, index);
    return Box::new(reader);
}

fn reader(vector: &Vec<i32>) -> Box<dyn Fn(usize) -> ()> {
    let retrieve = |vector: &Vec<i32>, index: usize| {
        match panic::catch_unwind(|| vector[index]) {
            Ok(element) => println!("Element {} is {}.", index, element),
            Err(_) => println!("There was an error trying to retrieve element {}.", index)
        };
    };

    return create_reader(&vector, retrieve);
}

fn optional_reader(vector: &Vec<i32>) -> Box<dyn Fn(usize) -> ()> {
    let retrieve = |vector: &Vec<i32>, index: usize| {
        match vector.get(index) {
            Some(element) => println!("Element {} is {}.", index, element),
            None => println!("There is no element {}.", index)
        };
    };

    return create_reader(&vector, retrieve);
}

pub fn read_vector_elements() {
    let v = vec![1, 2, 3, 4, 5];
    println!("The vector is {:?}.", v);

    fn run_reader_demo(description: &str, reader: Box<dyn Fn(usize) -> ()>) {
        println!("\n*** Retrieving using {} index retrieval ***\n", description);
        reader(2);
        reader(5);
    }

    run_reader_demo("straightforward", reader(&v));
    run_reader_demo("optional", optional_reader(&v));
}
