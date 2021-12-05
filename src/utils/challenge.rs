pub trait Challenge {
    // TODO: return a trait object
    fn new(lines: Vec<String>) -> Self;
    fn run(&self) -> Result<String, String>;
}
