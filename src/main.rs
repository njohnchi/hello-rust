use std::io::{stdout, Write};

fn main() {
    const TOTAL_COUNT: usize = 8;
    const SPACE: u8 = b' ';
    const COMMA: u8 = b',';
    const FIRST_DIGIT: u8 = b'0';
    const TOTAL_DIGITS: usize = (TOTAL_COUNT * (TOTAL_COUNT + 1) * (TOTAL_COUNT + 2)) / 6;
    const BUFFER_SIZE: usize = TOTAL_DIGITS * 5 - 1;

    let mut buffer = [0u8; BUFFER_SIZE];
    let mut index = 0;

    for i in 0..TOTAL_COUNT {
        for j in i + 1..=TOTAL_COUNT {
            for k in j + 1..=TOTAL_COUNT + 1 {
                buffer[index] = FIRST_DIGIT + i as u8;
                buffer[index + 1] = FIRST_DIGIT + j as u8;
                buffer[index + 2] = FIRST_DIGIT + k as u8;

                if index + 5 < BUFFER_SIZE {
                    buffer[index + 3] = COMMA;
                    buffer[index + 4] = SPACE;
                    index += 5;
                } else {
                    index += 3;
                }
            }
        }
    }

    buffer[BUFFER_SIZE - 1] = b'\n';

    stdout()
        .write_all(&buffer)
        .expect("Error writing to stdout");
}
