use std::io::{stdout, Write};

fn main() {
    const ALPHABET_COUNT: usize = 26;
    const NEWLINE: u8 = b'\n';

    let mut buffer: [u8; ALPHABET_COUNT * 2 + 1] = [0; ALPHABET_COUNT * 2 + 1];

    for i in 0..ALPHABET_COUNT {
        buffer[i] = b'a' + i as u8;
        buffer[ALPHABET_COUNT + i] = b'A' + i as u8;
    }

    buffer[ALPHABET_COUNT * 2] = NEWLINE;

    stdout()
        .write_all(&buffer)
        .expect("Error writing to stdout");
}
