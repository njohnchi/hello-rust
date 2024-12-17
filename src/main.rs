
fn main() {
    let random_number = rand::random::<i32>();
    
    if random_number > 0  {
        println!("{random_number} is positive");
    } else if random_number < 0 {
        println!("{random_number} is negative");
    } else {
        println!("{random_number} is zero");
    }
}