fn main() {
    println!("{:?}", powers_of_two(0));
    println!("{:?}", powers_of_two(1));
    println!("{:?}", powers_of_two(4));
}

fn powers_of_two(n: u8) -> Vec<u128> {
    (0..=n).map(|x| (2_u128).pow(x as u32)).collect()
}

