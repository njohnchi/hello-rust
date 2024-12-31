fn main() {
    print_last_digit(37);
    print_last_digit(0);
    let r = print_last_digit(-34564);
    println!("{r}");
}

fn print_alphabet() {
    for i in 0..26 {
        print!("{}", (b'a' + i) as char);
    }
    print!("\n");
}

fn print_alphabet_x10() {
    for _i in 0..10 {
        print_alphabet();
    }
}

fn is_lower(c: u8) -> u8 {
    if c >= b'a' && c <= b'z' {
        return 1;
    }
    0
}

fn is_alpha(c: u8) -> u8 {
    if c >= b'a' && c <= b'z' {
        return 1;
    }
    if c >= b'A' && c <= b'Z' {
        return 1;
    }
    0
}

fn print_sign(num: i64) -> i8 {
    if num > 0 {
        println!("+");
        return 1;
    }
    if num < 0 {
        println!("-");
        return -1;
    }

    println!("0");
    0
}

fn abs(num: i64) -> u64 {
    if num < 0 {
        return -num as u64;
    }
    num as u64
}

fn print_last_digit(num: i64) -> u64 {
    let mut last_digit = num % 10;
    if last_digit < 0 {
        last_digit *= -1;
    }
    println!("{last_digit}");
    last_digit as u64
}
