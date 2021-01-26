use crate::Challenge;
use regex::Regex;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Day2 {
    lines: Vec<String>,
}

#[derive(Debug)]
struct Password {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

impl FromStr for Password {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pat = Regex::new(r"^(\d+)-(\d+)\s(\w):\s(\w+)$")
            .expect("Failed to create regular expression.");
        let token = pat.captures(s).unwrap();

        Ok(Self {
            min: str::parse(token.get(1).unwrap().as_str()).unwrap(),
            max: str::parse(token.get(2).unwrap().as_str()).unwrap(),
            character: (token.get(3).unwrap().as_str()).chars().next().unwrap(),
            password: String::from(token.get(4).unwrap().as_str()),
        })
    }
}

// XXX: Use derive
impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min
            && self.max == other.max
            && self.character == other.character
            && self.password == other.password
    }
}

impl Password {
    fn is_at_position(&self, pos: usize) -> bool {
        match self.password.chars().nth(pos - 1) {
            Some(ch) => ch == self.character,
            None => false,
        }
    }

    fn is_valid_by_position(&self) -> bool {
        let lower = self.is_at_position(self.min);
        let upper = self.is_at_position(self.max);

        (lower && !upper) || (!lower && upper)
    }

    fn is_valid_by_count(&self) -> bool {
        let char_cnt = self
            .password
            .chars()
            .filter(|c| c == &self.character)
            .count();

        char_cnt >= self.min && char_cnt <= self.max
    }
}

impl Challenge for Day2 {
    fn new(lines: Vec<String>) -> Self {
        Day2 { lines }
    }

    fn run(&self) -> Result<String, String> {
        let correct_cnt = self
            .lines
            .iter()
            .filter(|l| Password::from_str(l).unwrap().is_valid_by_position())
            .count();

        Ok(format!(
            "Correct passwords: {} out of {}",
            correct_cnt,
            self.lines.len()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let act1 = Password::from_str("1-3 a: abcde").unwrap();
        let exp1 = Password {
            min: 1,
            max: 3,
            character: 'a',
            password: String::from("abcde"),
        };
        assert_eq!(act1, exp1);
    }
}
