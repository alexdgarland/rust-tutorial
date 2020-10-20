use add_one;
use rand;

fn main() {
    println!(
        "We're going to create a random number and then throw it away - {}!",
        rand::random::<u8>()
    );

    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
}
