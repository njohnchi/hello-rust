fn main() {
    println!("{}", boolean_to_string(false));
    println!("{}", boolean_to_string(true));
}

fn boolean_to_string(b: bool) -> String {
    match b {
        true => String::from("true"),
        false => String::from("false")
    }
}
