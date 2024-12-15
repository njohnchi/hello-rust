use std::io::{stdout, Write};

fn main() {
    let txt = b"Programming is like building a multilingual puzzle\n";

    match stdout().write(txt) {
        Ok(byte) => println!("written {} bytes to stdout", byte),
        Err(_) => println!("Could not write to stdout")
    }

    print!("with proper grammar, but the outcome is a piece of art,\n");
}
