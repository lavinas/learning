
pub(crate) fn main() {
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("x@test.com"),
        sign_in_count: 1,
    };
    println!("user1: {}, {}, {}, {}", user1.active, user1.username, user1.email, user1.sign_in_count);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
