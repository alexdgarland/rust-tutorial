use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn get_string() -> String {
    let mut stdin_string = String::new();

    io::stdin()
        .read_line(&mut stdin_string)
        .expect("Failed to read line");

    return stdin_string.trim().to_string();
}

fn get_guess() -> u8 {
    loop {
        println!("Please enter your guess.");

        let guess_string = get_string();
        println!("You guessed: {}", guess_string);

        match guess_string.parse::<u8>() {
            Ok(num) => return num,
            Err(_) => {
                println!("\"{}\" is not a valid 8-bit integer - please try again!", guess_string);
                continue;
            },
        };
    }
}

fn get_random() -> u8 {
    println!("Generating a secret number between 1 and 100...");
    return rand::thread_rng().gen_range(1, 101);
}


fn main() {
    println!("Guess the number!");

    let secret_number = get_random();

    loop {
        let guess = get_guess();

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess was too small!"),
            Ordering::Greater => println!("Your guess was too large!"),
            Ordering::Equal => {
                println!("Your guess was exactly right, you win!");
                break;
            },
        }
    }

}
