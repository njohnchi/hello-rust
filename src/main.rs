fn main() {
    println!("{}", add(4, 7));
    println!("{}", add(-4, 7));
    println!("{}", add(0, 0));
    println!("{}", add(4, -7));
    println!("{}", add(-4, -7));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}