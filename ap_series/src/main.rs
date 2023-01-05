#[derive(Debug)]
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

fn arithmetic_progression(start_at: u32, diff: u32) -> AP {
    AP {
        curr: start_at,
        common_diff: diff,
    }
}

fn shoes_in_size(series: Vec<u32>, element: u32) -> Vec<u32> {
    series.into_iter().filter(|s| s >= &element).collect()
}

fn main() {
    let series_start_at = 1u32;
    let common_difference = 3u32;

    println!("The first four terms of the Arithmetic progression sequence are: ");
    let mut arithmetic_series: Vec<u32> = Vec::new();
    for i in arithmetic_progression(series_start_at, common_difference).take(10) {
        arithmetic_series.push(i);
        print!("{} ", i);
    }

    println!("\nThe next four terms of the Arithmetic progression sequence are: ");
    for i in arithmetic_progression(series_start_at, common_difference)
        .skip(4)
        .take(4)
    {
        print!("{} ", i);
    }

    println!("\nPrint AP series starting from 11");
    let ap_series = shoes_in_size(arithmetic_series, 11u32);
    println!("{:?}", ap_series)
}