fn show_vector(type_description: &str, vector: &Vec<i32>) {
    println!("{} vector is - {:?}.", type_description, vector);
}

pub fn show_modified_vectors() {
    let mut initial_vector = vec![1, 2, 3, 4];
    show_vector("Initial", &initial_vector);

    for el in &mut initial_vector { *el += 1; }
    show_vector("Modified", &initial_vector);

    let mapped_vector: Vec<i32> = initial_vector.into_iter().map(|i| i * 2).collect();
    show_vector("Mapped", &mapped_vector);
}
