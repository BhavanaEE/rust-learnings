// Generics struct definitions

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_int = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    println!("{:?}\n{:?}\n{:?}\n",both_int,both_float,integer_and_float);
}