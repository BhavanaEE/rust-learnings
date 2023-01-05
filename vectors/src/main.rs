// vector collection

fn main() {
    let v1 = vec![1,2,3,4,5];
    for i in &v1 {
        println!("{}",i);
    }

    let mut v2 = vec![1,2,3,4,5];
    for i in &mut v2 {
        *i+=50;
    }
    for i in &v2 {
        println!("{}",i);
    }

    // get method

    let mut v3 = vec![1,2,3,4,5];
    let third = &v3[2];

    v3.push(56);
    //println!("third element is {}",third);

    match v2.get(2) {
        Some(x) => println!("element at index is {}",x),
        None => println!("no element at index"),
    }
}