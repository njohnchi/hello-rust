use std::io::{stdout, Write};

fn main() {
    const TOTAL_COUNT: usize = 9;
    const SPACE: u8 = b' ';
    const COMMA: u8 = b',';
    const FIRST_DIGIT: u8 = b'0';
    const TOTAL_DIGITS: usize = (TOTAL_COUNT * (TOTAL_COUNT + 1)) / 2;
    const BUFFER_SIZE: usize = TOTAL_DIGITS * 4 - 1;

    let mut buffer = [0u8; BUFFER_SIZE];
    let mut index = 0;

    for i in 0..TOTAL_COUNT {
        for j in i + 1..=TOTAL_COUNT {
            buffer[index] = FIRST_DIGIT + i as u8;
            buffer[index + 1] = FIRST_DIGIT + j as u8;

            if index + 4 < BUFFER_SIZE {
                buffer[index + 2] = COMMA;
                buffer[index + 3] = SPACE;
                index += 4;
            } else {
                index += 2;
            }
        }
    }

    buffer[BUFFER_SIZE - 1] = b'\n';

    stdout()
        .write_all(&buffer)
        .expect("Error writing to stdout");
}
