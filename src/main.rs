fn main() {
    println!("{}", are_you_playing_banjo("Ruth"));
    println!("{}", are_you_playing_banjo("Sam"));
    println!("{}", are_you_playing_banjo("Paul"));
    println!("{}", are_you_playing_banjo("Ruby"));
}

fn are_you_playing_banjo(name: &str) -> String {
    match &name[0..1] {
        "R" | "r" => format!("{} plays banjo", name),
        _ => format!("{} does not play banjo", name)
    }
}
