use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn load_file(filename: &str) -> Result<Vec<String>, std::io::Error> {
    println!("Loading file {}", filename);

    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    Ok(buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect())
}
