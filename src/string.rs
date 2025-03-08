use std::fmt::{Display, Formatter};

pub struct MyString {
    chars: [char; 100]
}

impl Display for MyString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.chars.len() {
            write!(f, "{}", self.chars[i])?
        }
        Ok(())
    }
}

impl MyString {
    pub fn new() -> Self {
        Self {
            chars: ['\0'; 100]
        }
    }

    pub fn from(string: &str) -> Self {
        let mut my_string = Self::new();
        for i in 0..my_string.chars.len() {
            if i >= string.len() { break }
            my_string.chars[i] = string.chars().collect::<Vec<_>>()[i];
        }
        my_string
    }
}