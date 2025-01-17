fn main() {
    println!("{}", rps("rock", "scissors"));
    println!("{}", rps("scissors", "rock"));
    println!("{}", rps("rock", "rock"));
}

fn rps(p1: &str, p2: &str) -> &'static str  {
    match (p1, p2) {
        ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => "Player 1 won!",
        ("rock", "paper") | ("paper", "scissors") | ("scissors", "rock") => "Player 2 won!",
        _ => "Draw!"
    }
}
