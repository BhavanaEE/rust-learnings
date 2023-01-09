pub fn add3(vec1: &mut Vec<i32>, number: i32) {
    for i in 0..3 {
        vec1[i]+=number;
    }
    println!("Add3 {:?}",vec1);
}

pub fn add2(vec1: &Vec<i32>, number: i32) -> Vec<i32> {
    let mut res= Vec::new();
    for i in 0..3 {
        res.push(vec1[i]+number);
    }
    res
}

pub fn add1(vec1: &Vec<i32>, number: i32) {
    println!("Add1");
    for i in 0..3 {
        println!("{:?} ", (vec1[i] + number));
    }
}
