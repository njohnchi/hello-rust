fn xo(string: &'static str) -> bool {
    string.chars().filter(|c| c.to_ascii_lowercase() == 'x').count()
        == string.chars().filter(|c| c.to_ascii_lowercase() == 'o').count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(xo("xo"), true);
        assert_eq!(xo("Xo"), true);
        assert_eq!(xo("xxOo"), true);
        assert_eq!(xo("xxxm"), false);
        assert_eq!(xo("Oo"), false);
        assert_eq!(xo("ooom"), false);
    }
}