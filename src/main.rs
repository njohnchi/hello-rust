fn main() {
    jack_bauer();
}

fn jack_bauer() {
    for i in 0..24 {
        for j in 0..60 {
            // println!("{i}:{j}");

            // if i < 10 {
            //     print!("0{i}:");
            // } else {
            //     print!("{i}:");
            // }
            // if j < 10 {
            //     print!("0{j}\n");
            // } else {
            //     print!("{j}\n");
            // }

            println!("{i:02}:{j:02}");
        }
    }
}