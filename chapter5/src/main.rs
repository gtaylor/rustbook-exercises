#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
}

impl User {
    fn is_active(&self) -> bool {
        self.active
    }
}

fn build_user(email: String, username: String) -> User {
    // Var name matches field name, serves as Init shorthand.
    User {
        active: true,
        username,
        email,
    }
}

fn main() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    user1.username = String::from("testuser123");
    println!("Hello, {0}!", user1.username);

    let user2 = User {
        // Override specific field(s)
        email: dbg!(String::from("another@example.com")),
        // Inherits other fields from donor
        ..user1
    };
    dbg!(user2.is_active());
    // Can no longer use the email/username values in user1 since they moved via
    // stack-only data copy. Other values are fine since they can be copied
    // rather than moved.
    println!("Oh hai, {:#?}", user2);
}
