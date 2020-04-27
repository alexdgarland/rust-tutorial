use std::fmt;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl fmt::Display for User {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Username: {}\nEmail: {}\nSign-In Count: {}\nActive: {}\n",
            self.username,
            self.email,
            self.sign_in_count,
            self.active
        )
    }
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

pub fn demo_user() {

    let mut user = build_user(
        String::from("someone@example.com"),
        String::from("someone"),
    );

    println!("{}", user);

    println!("Updating user email address...\n");
    user.email = String::from("my_new_email@example.com");

    println!("{}", user);

    println!("Creating new user with some properties same as old user.");
    let user2 = User {
        username: String::from("newuser"),
        email: String::from("newuser@example.com"),
        ..user
    };
    println!("{}", user2);

}