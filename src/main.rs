fn main() {
    print_fib();
}

fn print_fib() {
    let mut first: u64 = 1;
    let mut second: u64 = 2;
    for i in 0..50 {
        if i == 0 {
            print!("{first}");
            continue;
        }
        if i == 1 {
            print!(", {second}");
            continue;
        }
        let sum: u64 = first + second;
        print!(", {}", sum);
        first = second;
        second = sum;
    }
    print!("\n");
}
