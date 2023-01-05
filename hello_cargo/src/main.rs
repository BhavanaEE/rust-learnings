use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {
    println!("Guess a random number :)");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret number is {secret_number}");
    loop {
        println!("Provide an input number to guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("your guess is {guess}");



    let guess1:i32 = guess.trim().parse().expect("please type a number");
    match guess1.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {println!("you win");
    break;}

    }
    }
}