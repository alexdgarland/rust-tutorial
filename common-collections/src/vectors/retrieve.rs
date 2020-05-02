use std::panic;

fn retrieve(vector: &Vec<i32>, index: usize) {
    match panic::catch_unwind(|| vector[index]) {
        Ok(element) => println!("Element {} is {}.", index, element),
        Err(_) => println!("There was an error trying to retrieve element {}.", index)
    };
}

fn retrieve_optional(vector: &Vec<i32>, index: usize) {
    match vector.get(index) {
        Some(element) => println!("Element {} is {}.", index, element),
        None => println!("There is no element {}.", index)
    };
}

pub fn read_vector_elements() {
    let v = vec![1, 2, 3, 4, 5];
    println!("The vector is {:?}.", v);

    let run_retrieval_demo = |description: &str, retriever: fn(&Vec<i32>, usize)| {
        println!("\n*** Retrieving using {} index retrieval ***\n", description);
        let reader = |index: usize| retriever(&v, index);
        reader(2);
        reader(5);
    };

    run_retrieval_demo("straightforward", retrieve);
    run_retrieval_demo("optional", retrieve_optional);
}
