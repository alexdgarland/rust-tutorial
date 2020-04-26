use std::fmt;

pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
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

pub fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
