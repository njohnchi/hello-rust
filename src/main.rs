fn main() {
    print_alphabet_x10()
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
