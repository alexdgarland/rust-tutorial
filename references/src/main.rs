
fn alter(s: &mut String) {
    s.push_str(", world!");
}

fn show_array(array: &[u8]) {
    println!("Printing out elements of array:");
    for i in array.iter() {
        println!("{}", i);
    }
}

fn main() {
    let mut s1 = String::from("Hello");
    alter(&mut s1);
    println!("{}", s1);
    // let mut s2 = s1;
    // let mut s3 = s2;

    let mut array = [1, 2, 3, 4, 5];
    show_array(&array);

    array[1] = 100;
    show_array(&array);

    let sub_array = &array[1..4];
    show_array(&sub_array);
}
