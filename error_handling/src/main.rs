// Error handling

use std::fs::File;

fn main() {
    let mut f = File::open("hello.txt");

    // shadowing to redeclare f
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("problem while opening a file {:?}",error),
    };
}
