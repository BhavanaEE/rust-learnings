fn main() {
    let s1 = String::from("hello");
    let s2 = references_borrow(s1);
    println!("{}", s2.0);
}

fn references_borrow(s1: String) -> (String, usize) {
    println!("inside func {}", s1);
    let length = s1.len();
    (s1, length)
}

fn first_word(s: &String) -> usize {
    let s_as_bytes = s.as_bytes();
    for (i, &item) in s_as_bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
