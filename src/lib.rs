pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod input;

pub trait Challenge {
    // TODO: return a trait object
    fn new(lines: Vec<String>) -> Self;
    fn run(&self) -> Result<String, String>;
}
