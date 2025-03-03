fn main() {
    println!("{}", disemvowel("This website is for losers LOL!"));
}

fn disemvowel(s: &str) -> String {
    s.chars().filter(|&c| {
        !"aeiouAEIOU".contains(c)
    }).collect()
}
