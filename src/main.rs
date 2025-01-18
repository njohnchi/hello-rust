fn main() {
    println!("{:?}", remove_every_other(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
}

fn remove_every_other(arr: &[u8]) -> Vec<u8> {
    arr.iter().step_by(2).copied().collect()
}

