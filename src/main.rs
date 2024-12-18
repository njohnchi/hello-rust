use std::io::{stdout, Write};

fn main() {
    const TOTAL_COUNT: usize = 10;
    const SPACE: u8 = b' ';
    const COMMA: u8 = b',';
    const FIRST_DIGIT: u8 = b'0';
    const BUFFER_SIZE: usize = TOTAL_COUNT * 3 - 1;

    let mut buffer = [0u8; BUFFER_SIZE];

    buffer[0] = FIRST_DIGIT;

    for i in 0..TOTAL_COUNT - 1 {
        let index = i * 3;
        buffer[index + 1] = COMMA;
        buffer[index + 2] = SPACE;
        buffer[index + 3] = FIRST_DIGIT + 1 + i as u8;
    }

    buffer[BUFFER_SIZE - 1] = b'\n';

    stdout()
        .write_all(&buffer)
        .expect("Error writing to stdout");
}
