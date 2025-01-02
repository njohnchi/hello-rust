fn main() {
    print_times_table(3);
    print!("\n");
    print_times_table(5);
    print!("\n");
    print_times_table(98);
    print!("\n");
    print_times_table(12);
}

fn print_times_table(n: i32) {
    if n < 0 || n > 15 {
        return;
    }

    for i in 0..=n {
        for j in 0..=n {
            if j == 0 {
                print!("{}", i * j);
                continue;
            }
            print!(",{:3}", i * j);
        }
        print!("\n");
    }
}