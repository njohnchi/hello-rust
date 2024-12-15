use std::io::{stdout, Write, stdin, Read};

fn main() {
    let mut buffer: [u8; 1000] = [0; 1000];

    match stdin().read(&mut buffer) {
        Ok(bytes) => {
            println!("Read {} bytes from stdin", bytes);
            let input = &buffer[0..bytes];

            match stdout().write(input) {
                Ok(bytes) => println!("written {} bytes to stdout", bytes),
                Err(_) => eprintln!("Could not write to stdout")
            }
        }
        Err(_) => eprintln!("Could not read from stdin")
    }
}
