// Strings are stored as collection of utf-8 encoded bytes

use std::fmt::format;
use unicode_segmentation::UnicodeSegmentation;
fn main() {
    let s1 = String::new(); // initialising empty string
    let s2 = "hey bhavana, how are you?";
    let s3 = s2.to_string();
    let s4 = s1.to_string();
    let mut s5 = String::from("Bhavana");

    println!("{}\n{}\n{}\n{}\n{}",s1,s2,s3,s4,s5);

    
    // grow and shrink the size
    s5.push_str(" chivukula");
    s5.push('!');
    println!("{}",s5);

    let s6 = s5+ s2; 
    println!("{}",s6);


    let s11 = String::from("Hello ");
    let s12 = String::from("world");
    // let s13 = s11 + &s12; // moving ownership of s11 to s13 and taking all characters in s12 to s3. So we need to borrow.
    let s13 = format!("{}{}",s11,s12); // doesn't take ownership of these strings
    println!("{}",s13);


    // INDEXING INTO A STRING
    // Bytes, Scalar values, Grapheme clusters 

    for b in s11.bytes() {
        println!("In bytes{}", b)
    }
    for c in s11.chars() {
        println!("In chars{}", c)
    }
    //need to have unicode segmentation crate
    for g in s11.graphemes(true) {
        println!("In graphemes{}", g)
    }
}
