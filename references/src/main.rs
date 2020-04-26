
fn alter(s: &mut String) {
    s.push_str(", world!");
}

fn main() {
    let mut s1 = String::from("Hello");
    alter(&mut s1);
    println!("{}", s1);
    // let mut s2 = s1;
    // let mut s3 = s2;
}
