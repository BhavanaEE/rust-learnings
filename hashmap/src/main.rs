// hashmap collection

use std::collections::HashMap;

fn main() {
    let blue_team = String::from("blue");
    let green_team = String::from("green");

    let mut scores = HashMap::new();
    scores.insert(blue_team, 10);
    scores.insert(green_team, 30);

    // println!("{}",blue_team); //cannot borrow

    let team_name = String::from("blue");
    let score = scores.get(&team_name);

    for (key,value) in scores {
        println!("{}{}",key,value);
    }
}