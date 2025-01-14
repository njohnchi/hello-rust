fn main() {
    println!("{}", even_or_odd(2));
    println!("{}", even_or_odd(31));
    println!("{}", even_or_odd(-8));
    println!("{}", even_or_odd(-53));
}

fn even_or_odd(number: i32) -> &'static str {
    if number % 2 == 0 {
        "Even"
    } else {
        "Odd"
    }
}
