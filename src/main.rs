use advent_of_code::input::Input;
use advent_of_code::Challenge;
use advent_of_code::{day1, day2, day3};
use std::env;
use std::process;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let input = Input::load(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let result = match input.day {
        1 => {
            let challenge = day1::Day1::new(input.lines);
            challenge.run()
        }
        2 => {
            let challenge = day2::Day2::new(input.lines);
            challenge.run()
        }
        3 => {
            let challenge = day3::Day3::new(input.lines);
            challenge.run()
        }
        _ => Err(format!("Invalid day: {}", input.day)),
    }
    .unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1);
    });

    println!("The result for day {}:\t\n{}", input.day, result);
}
