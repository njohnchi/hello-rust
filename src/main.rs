fn main() {
    println!("{}", basic_op('+', 4, 7));
    println!("{}", basic_op('-', 11, 8));
    println!("{}", basic_op('*', 2, 6));
    println!("{}", basic_op('/', 45, 5));
}

fn basic_op(operator: char, value1: i32, value2: i32) -> i32 {
    match operator {
        '+' => value1 + value2,
        '-' => value1 - value2,
        '*' => value1 * value2,
        '/' => value1 / value2,
        _ => panic!("wrong operator")
    }
}
