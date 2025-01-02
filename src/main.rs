fn main() {
    print_to_98(0);
    print_to_98(98);
    print_to_98(111);
    print_to_98(81);
    print_to_98(-10);
}

fn print_to_98(n: i32) {
    if n > 98 {
        for i in (98..=n).rev() {
            if i == n {
                print!("{i}");
                continue;
            }
            print!(", {i}");
        }
        print!("\n");
    } else {
        for i in n..=98 {
            if i == n {
                print!("{i}");
                continue;
            }
            print!(", {i}");
        }
        print!("\n");
    }
}