struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: false,
        sign_in_count: 1,
    };

    User {
        email,
        username,
        ..user2
    }
}

fn main() {
    let user1 = build_user(String::from("zihua@qq.com"), String::from("zihua"));
    println!("{} {}", user1.username, user1.sign_in_count);
}
