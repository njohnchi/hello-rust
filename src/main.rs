fn main() {
    println!("{}", rps("rock", "scissors"));
    println!("{}", rps("scissors", "rock"));
    println!("{}", rps("rock", "rock"));
}

fn rps(p1: &str, p2: &str) -> &'static str  {
    match p1 {
        "rock" => {
            match p2 {
                "rock" => "Draw!",
                "paper" => "Player 2 won!",
                "scissors" => "Player 1 won!",
                _ => panic!("Invalid option")
            }
        },
        "paper" => {
            match p2 {
                "rock" => "Player 1 won!",
                "paper" => "Draw!",
                "scissors" => "Player 2 won!",
                _ => panic!("Invalid option")
            }
        },
        "scissors" => {
            match p2 {
                "rock" => "Player 2 won!",
                "paper" => "Player 1 won!",
                "scissors" => "Draw!",
                _ => panic!("Invalid option")
            }
        },
        _ => panic!("Invalid option")
    }
}
