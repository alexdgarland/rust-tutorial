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

fn reader(vector: &Vec<i32>) -> Box<dyn Fn(usize) -> ()> {
    let vc = vector.clone();
    let reader = move |index: usize|
        match panic::catch_unwind(|| vc[index]) {
            Ok(element) => println!("Element {} is {}.", index, element),
            Err(_) => println!("There was an error trying to retrieve element {}.", index)
        };
    return Box::new(reader);
}

fn optional_reader(vector: &Vec<i32>) -> Box<dyn Fn(usize) -> ()> {
    let vc = vector.clone();
    let reader = move |index: usize|
        match vc.get(index) {
            Some(element) => println!("Element {} is {}.", index, element),
            None => println!("There is no element {}.", index)
        };
    return Box::new(reader);
}

pub fn read_vector_elements() {
    let v = vec![1, 2, 3, 4, 5];
    println!("The vector is {:?}.", v);

    let announce = |description: &str|
        println!("*** Retrieving using {} index retrieval ***", description);

    announce("straightforward");
    let read_element = reader(&v);
    read_element(2);
    read_element(5);

    announce("optional");
    let read_optional_element = optional_reader(&v);
    read_optional_element(2);
    read_optional_element(5);
}