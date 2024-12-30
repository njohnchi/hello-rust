fn main() {
    let mut r = is_alpha(b'H');
    print!("{r}");
    r = is_alpha(b'o');
    print!("{r}");
    r = is_alpha(108);
    print!("{r}");
    r = is_alpha(b';');
    print!("{r}");
    print!("\n");
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
