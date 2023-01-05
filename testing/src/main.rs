pub fn greeting(name: &str) -> String {
    format!("Hello! {}", name)
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_value_can_hold_small_value() {
        let large_rect = Rectangle {
            width: 6,
            height: 7,
        };
        let small_rect = Rectangle {
            width: 5,
            height: 6,
        };

        assert!(large_rect.can_hold(&small_rect));
    }

    #[test]
    fn greeting_contains_name() {
        let greet = greeting("Bhavana");

        // assert with custom message
        assert!(greet.contains("na"),"Greeting doesn't contain name, value was {}",greet); 
    }

    #[test]
    fn get_sum() -> Result<(),String> {
        if 2+8 == 10 {
            Ok(())
        } else {
            Err(String::from("Invalid addition operation"))
        }
    }
}
