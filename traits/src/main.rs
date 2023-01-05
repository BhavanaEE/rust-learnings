// Traits

use std::fmt::format;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Person {
    pub name: String,
    pub age: u32,
    pub gender: char,
}

impl Summary for Person {
    fn summarize(&self) -> String {
        format!("Name of the person is:{} and age is:{}",self.name,self.age)
    }
}

pub struct Student {
    pub roll_number: u32,
    pub name: String,
    pub class: u32,
    pub marks_sheet: u32
}

impl Summary for Student {
    fn summarize(&self) -> String {
        format!("Student name is:{} and roll number is:{}",self.name,self.roll_number)
    }
}

fn main() {
    let per = Person {
        name: String::from("Bhavana"),
        age:23,
        gender: 'F',
    };

    let stu = Student {
        roll_number: 66,
        name: String::from("Bhavana"),
        class: 1,
        marks_sheet: 95,
    };

    println!("Person details = {}",per.summarize());
    println!("Student details = {}",stu.summarize());
}