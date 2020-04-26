mod user;

fn main() {
    let mut user = user::build_user(
        String::from("someone@example.com"),
        String::from("someone"),
    );

    println!("{}", user);

    println!("Updating user email address...\n");
    user.email = String::from("my_new_email@example.com");

    println!("{}", user);

    println!("Creating new user with some properties same as old user.");
    let user2 = user::User {
        username: String::from("newuser"),
        email: String::from("newuser@example.com"),
        ..user
    };
    println!("{}", user2);
}
