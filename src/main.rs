fn main() {
    let random_number = rand::random::<u32>();
    let last_digit = random_number % 10;

    print!("Last digit of {random_number} is {last_digit} ");
    if last_digit > 5 {
        print!("and is greater than 5\n")
    } else if last_digit > 0 {
        print!("and is less than 6 and not 0\n")
    } else {
        print!("and is 0\n")
    }
}