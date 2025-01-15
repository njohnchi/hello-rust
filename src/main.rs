fn main() {
    println!("{}", solution("false"));
    println!("{}", solution("true"));
}

fn solution(phrase: &str) -> String {
    let mut result = String::new();
    for c in phrase.chars().rev() {
        result.push(c);
    }
    result
}
