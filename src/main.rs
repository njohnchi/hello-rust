fn main() {
    println!("{:?}", count_by(1, 10));
    println!("{:?}", count_by(2, 5));
}

fn count_by(x: u32, n: u32) -> Vec<u32> {
    (1..=n).map(|i| i * x).collect()
}

