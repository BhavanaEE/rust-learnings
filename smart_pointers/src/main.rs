struct Greet {
    data: String
}

impl Drop for Greet {
    fn drop(&mut self) {
        println!("Dropping greet data with value : {}",self.data);
    }
}
fn main() {
    let greet = Greet {data: String::from("Hey Bhavana")};
    println!("Customer smart pointer Greet created.....");
    drop(greet);
    println!("CustomSmartPointer dropped before the end of main.");
}