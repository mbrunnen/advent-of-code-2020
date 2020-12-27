use advent_of_code::input::load_file;
use std::env;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Please give exactly one positional argument!")
    }

    let filename = &args[1];

    let lines: Vec<String> = load_file(filename)?;

    println!("Loaded {} lines", lines.len());

    let nums: Vec<i32> = lines
        .iter()
        .map(|l| l.parse::<i32>().unwrap_or(0))
        .collect();

    if let Some((x, y, z)) = find_three_summands(&nums[0], &nums[1..], 2020) {
        println!("The product is: {} * {} * {} = {}", x, y, z, x * y * z);
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

fn find_three_summands(cur: &i32, rest: &[i32], sum: i32) -> Option<(i32, i32, i32)> {
    if rest.is_empty() {
        return None;
    }

    let diff = sum - cur;
    if let Some((add1, add2)) = find_two_summands(&rest[0], &rest[1..], diff) {
        println!("{} + {} + {} = {}", cur, add1, add2, cur + add1 + add2);
        return Some((*cur, add1, add2));
    }

    find_three_summands(&rest[0], &rest[1..], sum)
}
