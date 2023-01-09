mod calculator;

use calculator::{add1,add2,add3};

fn main() {
    let mut vec1 = vec![1,2,3];
    let number = 10;
    add1(&vec1,number);
    let result = add2(&vec1,number);
    println!("Add2 {:?}",result);
    add3(&mut vec1,number);
}

