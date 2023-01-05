struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

// implementation block (method)
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

//Associated functions
impl Rectangle {
    fn Square(dimension: u32) -> Self {
        Self { width: dimension, height: dimension }
    }
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

    let rect1 = Rectangle {
        width: 2,
        height: 4,
    };

    println!("{}", area(rect));

    // implementation

    // method
    println!("area of rect is {} ", rect1.area());

    // associated func code
    let sq = Rectangle::Square(3);
    println!("area of rect {}",sq.area());

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