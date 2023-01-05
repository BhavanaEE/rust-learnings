// Traits

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Default implementation..")
    }
    fn summarize_author(&self) -> String;
}

// trait bound
pub fn notify_studemt_exam_marks<T,U>(item1:T,item2:U) -> String
    where T: Summary,
          U: Summary
{
    format!("exam marks notificaation for student name: {} and {}",item1.summarize_author(),item2.summarize())
}

pub struct Person {
    pub name: String,
    pub age: u32,
    pub gender: char,
}

impl Summary for Person {
    fn summarize_author(&self) -> String {
        format!("@ author - {}",self.name)
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
    fn summarize_author(&self) -> String {
        format!("@ author {}",self.name)
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

    println!("Person details: {} {}",per.summarize_author(),per.summarize()); // Person details = Default implementation..
    println!("Student details: {} {}",per.summarize_author(),stu.summarize()); // Student details = Student name is:Bhavana and roll number is:66

    // trait bound
    println!("{}",notify_studemt_exam_marks(per, stu));
}