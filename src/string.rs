use std::fmt::{Display, Formatter};

pub struct MyString {
    chars: Vec<char>
}

impl Display for MyString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in self.chars.iter() {
            write!(f, "{}", c)?
        }
        Ok(())
    }
}

impl MyString {
    pub fn new() -> Self {
        Self {
            chars: Vec::new(),
        }
    }

    pub fn from(string: &str) -> Self {
        Self {
            chars: string.chars().collect()
        }
    }

    pub fn push(&mut self, ch: char) {
        self.chars.push(ch)
    }

    pub fn push_str(&mut self, string: &str) {
        string.chars().for_each(|c| self.chars.push(c));
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }
}

#[cfg(test)]
mod test_string {
    use super::*;

    #[test]
    fn it_creates_empty_string() {
        assert_eq!(format!("{}", MyString::new()), "");
    }

    #[test]
    fn it_creates_string_from_str() {
        assert_eq!(format!("{}", MyString::from("hello")), "hello");
        assert_eq!(format!("{}", MyString::from("world")), "world");
        assert_eq!(format!("{}", MyString::from("  ")), "  ");
        assert_eq!(format!("{}", MyString::from("./")), "./");
        assert_eq!(format!("{}", MyString::from("")), "");
    }

    #[test]
    fn it_push_str_to_my_string() {
        let mut my_str = MyString::from("hel");
        assert_eq!(format!("{}", my_str), "hel");
        my_str.push_str("lo");
        assert_eq!(format!("{}", my_str), "hello");
        my_str.push_str(" ");
        my_str.push_str("");
        my_str.push_str("world");
        assert_eq!(format!("{}", my_str), "hello world");
    }

    #[test]
    fn it_push_char_to_my_string() {
        let mut my_str = MyString::from("hel");
        assert_eq!(format!("{}", my_str), "hel");
        my_str.push('l');
        my_str.push('o');
        assert_eq!(format!("{}", my_str), "hello");
        my_str.push(' ');
        my_str.push('w');
        my_str.push('o');
        my_str.push('r');
        my_str.push('l');
        my_str.push('d');
        assert_eq!(format!("{}", my_str), "hello world");
    }

    #[test]
    fn it_returns_length_of_string() {
        let mut my_str = MyString::new();
        assert_eq!(my_str.len(), 0);
        my_str.push_str("hel");
        assert_eq!(my_str.len(), 3);
        my_str.push_str("lo");
        assert_eq!(my_str.len(), 5);
    }
}
