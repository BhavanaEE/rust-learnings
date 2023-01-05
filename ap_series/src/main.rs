struct AP {
    curr: u32,
    common_diff: u32,
}

impl Iterator for AP {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = current + self.common_diff;
        Some(current)
    }
}

fn arithmetic_progression() -> AP {
    AP { curr: 1, common_diff: 3 }
}

fn main() {

    println!("The first four terms of the Arithmetic progression sequence are: ");
    for i in arithmetic_progression().take(4) {
        print!("{} ", i);
    }

    println!("\nThe next four terms of the Arithmetic progression sequence are: ");
    for i in arithmetic_progression().skip(4).take(4) {
        print!("{} ", i);
    }

}