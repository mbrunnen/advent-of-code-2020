use crate::Challenge;

pub struct Day1 {
    lines: Vec<String>,
}

impl Challenge for Day1 {
    fn new(lines: Vec<String>) -> Self {
        Day1 { lines }
    }

    fn run(&self) -> Result<String, String> {
        let nums: Vec<i32> = self
            .lines
            .iter()
            .map(|l| l.parse::<i32>().unwrap_or(0))
            .collect();

        if let Some((x, y, z)) = find_three_summands(&nums[0], &nums[1..], 2020) {
            Ok(format!(
                "The product is: {} * {} * {} = {}",
                x,
                y,
                z,
                x * y * z
            ))
        } else {
            Err(String::from("No pair was found!"))
        }
    }
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
