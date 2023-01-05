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
    use super::Rectangle;

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
}
