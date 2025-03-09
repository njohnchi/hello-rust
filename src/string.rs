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
}
