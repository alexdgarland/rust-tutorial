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
        info!("Created vector - {:?}", v);
    }
}
