use std::io::{stdout, Write};

fn main() {
    let mut buffer = [0u8; 17];
    for i in 0..16 {
        if i < 10 {
            buffer[i] = b'0' + i as u8;
        } else {
            buffer[i] = b'a' + (i % 10) as u8;
        }
    }
    buffer[16] = b'\n';

    stdout()
        .write_all(&buffer)
        .expect("Error writing to stdout");
}