use std::env;
use std::fs::File;
use std::io;
use std::io::{prelude::*, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Please give exactly one positional argument!")
    }

    let filename = &args[1];

    println!("Loading file {}", filename);

    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    println!("Loaded {} lines", lines.len());

    let nums: Vec<i32> = lines
        .iter()
        .map(|l| l.parse::<i32>().unwrap_or(0))
        .collect();

    for num in nums {
        println!("{:?}", num);
    }

    Ok(())
}
