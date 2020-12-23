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

    if let Some((x, y)) = find_two_summands(&nums[0], &nums[1..], 2020) {
        println!("The product is: {} * {} = {}", x, y, x * y);
    } else {
        println!("No pair was found!");
    };

    Ok(())
}

fn find_two_summands(cur: &i32, rest: &[i32], sum: i32) -> Option<(i32, i32)> {
    if rest.is_empty() {
        return None;
    }

    for add in rest {
        if cur + add == sum {
            println!("{} + {} = {}", cur, add, cur + add);
            return Some((*cur, *add));
        }
    }

    find_two_summands(&rest[0], &rest[1..], sum)
}

fn find_sum(data: &mut [i32]) -> Option<(i32, i32, i32)> {
    for i in 0..data.len() {
        for j in 0..data.len() {
            for k in 0..data.len() {
                if let (Some(a), Some(b), Some(c)) = (data.get(i), data.get(j), data.get(k)) {
                    if a + b + c == 2020 {
                        println!("{} + {} + {} = {}", a, b, c, a + b + c);
                        return Some((data[i], data[j], data[k]));
                    }
                }
            }
        }
    }

    None
}
