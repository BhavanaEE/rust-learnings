struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("Bhavana"),
        email: String::from("qwerty@123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = build_user(String::from("anki@123"), String::from("ankita"));

    // string init
    let user3 = User {
        email: String::from("asdf@123"),
        username: String::from("asdf"),
        ..user2
    };
    println!("{}\n{}\n{}", user1.email, user2.sign_in_count, user3.active);

    // struct tuple

    let rect = (30, 40);
    println!("{}", area(rect));
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: false,
    }
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}