// hashmap collection

//updating hashmap
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("blue"), 30);

    println!("{:?}",scores.get("blue")); // Some(30) - overrides

    scores.entry(String::from("yellow")).or_insert(20);
    scores.entry(String::from("yellow")).or_insert(68);

    println!("{:?}",scores.get("yellow")); // Some(20)
}