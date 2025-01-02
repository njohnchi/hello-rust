fn main() {
    times_table();
}

fn times_table() {
    for i in 0..=9 {
        for j in 0..=9 {
            if j == 0 {
                print!("{}", i * j);
                continue;
            }
            print!(",{:3}", i * j);
        }
        print!("\n");
    }
}