use std::io::{stdout, Write};

fn main() {
    const TOTAL_COUNT: usize = 16;
    let mut buffer = [0u8; TOTAL_COUNT + 1];

    for i in 0..TOTAL_COUNT {
        if i < 10 {
            buffer[i] = b'0' + i as u8;
        } else {
            buffer[i] = b'a' + (i % 10) as u8;
        }
    }
    buffer[TOTAL_COUNT] = b'\n';

    stdout()
        .write_all(&buffer)
        .expect("Error writing to stdout");
}