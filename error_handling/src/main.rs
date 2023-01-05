// Error handling

use std::{fs::File, error::Error};
use std::io::ErrorKind;
fn main() {
    let mut f = File::open("hello.txt");

    // shadowing to redeclare f
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}",e),
            }

            other_error => {
                panic!("problem opening the file {:?}",other_error)
            }
        }
    };

    //unwrap
    let f1 = File::open("hello.txt").unwrap();

    // expect
    let f2 = File::open("hello.txt")
        .expect("No file named hello.txt");

}