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

    // STORING ENUM VARIANTS INSIDE VECTOR

    #[derive(Debug)] // adding debug trait
    enum SS {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SS::Int(3),
        SS::Float(6.89),
        SS::Text(String::from("Hello bhavana")),
    ];

    for i in &row {
        println!("elements in row are {:?}",i);
    }

    match &row[1] {
        SS::Int(i) => println!("value of integer is {}",i),
        SS::Float(i) => println!("value of string is {}",i),
        SS::Text(i) => println!("value of text is {}",i),

    }
}