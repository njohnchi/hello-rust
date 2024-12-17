use std::io::{stdout, Write};

fn main() {
    let mut buffer = [0u8; 11];
    for i in 0..10 {
        buffer[i] = b'0' + i as u8;
    }
    buffer[10] = b'\n';

    stdout()
        .write_all(&buffer)
        .expect("Error writing to stdout");
}
