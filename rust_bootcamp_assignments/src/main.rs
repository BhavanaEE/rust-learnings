fn main() {
    let mut vec1 = vec![1,2,3];
    let number = 10;
    add1(&vec1,number);
    let result = add2(&vec1,number);
    println!("Add2 {:?}",result);
    add3(&mut vec1,number);
}

fn add3(vec1: &mut Vec<i32>, number: i32) {
    for i in 0..3 {
        vec1[i]+=number;
    }
    println!("Add3 {:?}",vec1);
}

fn add2(vec1: &Vec<i32>, number: i32) -> Vec<i32> {
    let mut res= Vec::new();
    for i in 0..3 {
        res.push(vec1[i]+number);
    }
    res
}

fn add1(vec1: &Vec<i32>, number: i32) {
    println!("Add1");
    for i in 0..3 {
        println!("{:?} ", (vec1[i] + number));
    }
}
