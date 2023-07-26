//#[derive(Debug)]
use std::fmt;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    // Another associated function, taking two arguments:
    fn new(username: String) -> User {
        User { active: true, username: username, email: "undefined".to_string(), sign_in_count: 0  }
    }
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
         .field("active", &self.active)
         .field("username", &self.username)
         .field("email", &self.email)
         .field("sign_in_count", &self.sign_in_count)
         .finish()
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
} }

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    dbg!(user1);

    let user2 = build_user("dd@gg.com".to_string(), "gugus".to_string());
    dbg!(user2);

    let user3 = User::new("Michael Schenk".to_string());
    dbg!(user3);

}
