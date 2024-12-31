fn main() {
    let mut r = print_sign(98);
    println!("{r}");
    r = print_sign(0);
    println!("{r}");
    r = print_sign(0xff);
    println!("{r}");
    r = print_sign(-32);
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
