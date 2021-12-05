mod utils;
mod days;

use utils::input::Input;
use std::env;
use std::process;
use days::{day1::*, day2::*, day3::*, day4::*};
use crate::utils::challenge::Challenge;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let input = Input::load(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let result = match input.day {
        1 => {
            let challenge = Day1::new(input.lines);
            challenge.run()
        }
        2 => {
            let challenge = Day2::new(input.lines);
            challenge.run()
        }
        3 => {
            let challenge = Day3::new(input.lines);
            challenge.run()
        }
        4 => {
            let challenge = Day4::new(input.lines);
            challenge.run()
        }
        _ => unimplemented!("Invalid day: {}", input.day),
    }
    .unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });

    println!("The result for day {}:\t\n{}", input.day, result);
}
