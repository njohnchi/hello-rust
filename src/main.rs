fn main() {
    println!("{}", are_you_playing_banjo("Ruth"));
    println!("{}", are_you_playing_banjo("Sam"));
    println!("{}", are_you_playing_banjo("Paul"));
    println!("{}", are_you_playing_banjo("Ruby"));
}

fn are_you_playing_banjo(name: &str) -> String {
    let mut name = String::from(name);
    let b_name = name.as_bytes();
    if b_name[0] == b'R' || b_name[0] == b'r' {
        name.push_str(" plays banjo");
        return name;
    }
    name.push_str(" does not plays banjo");
    return name;
}
