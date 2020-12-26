use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

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

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min
            && self.max == other.max
            && self.character == other.character
            && self.password == other.password
    }
}

impl Password {
    fn is_valid_by_count(&self) -> bool {
        let char_cnt = self
            .password
            .chars()
            .filter(|c| c == &self.character)
            .count();

        char_cnt >= self.min && char_cnt <= self.max
    }
}

fn load_file(filename: &str) -> Result<Vec<String>, std::io::Error> {
    println!("Loading file {}", filename);

    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    Ok(buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect())
}

pub fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Please give exactly one positional argument!")
    }

    let filename = &args[1];

    let lines: Vec<String> = load_file(filename)?;

    let correct_cnt = lines
        .iter()
        .filter(|l| Password::from_str(l).unwrap().is_valid_by_count())
        .count();

    println!("Correct passwords: {} out of {}", correct_cnt, lines.len());

    Ok(())
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
