fn main() {
    println!("{:?}", remove_every_other(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
}

fn remove_every_other(arr: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut index = 0;
    while index < arr.len() {
        if index % 2 == 0 {
            result.push(arr[index]);
        }
        index += 1;
    }
    result
}

