use advent_of_code::input::load_file;
use std::env;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
struct Slope {
    map: Vec<Vec<bool>>,
    x: usize,
    y: usize,
    dx: i32,
    dy: i32,
}

impl Iterator for Slope {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.x = ((self.x as i32 + self.dx) % self.map[0].len() as i32) as usize;

        if self.y as i32 + self.dy < 0 || self.y as i32 + self.dy >= self.map.len() as i32 {
            return None;
        } else {
            self.y = (self.y as i32 + self.dy) as usize;
        };

        Some(self.map[self.y as usize][self.x as usize])
    }
}

impl Slope {
    fn new(map: &[String], dx: i32, dy: i32) -> Self {
        Slope {
            map: map
                .iter()
                .map(|s| s.chars().map(|c| c == '#').collect::<Vec<bool>>())
                .collect::<Vec<Vec<bool>>>(),
            x: 0,
            y: 0,
            dx,
            dy,
        }
    }
}

pub fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::new(
            ErrorKind::Other,
            "Please give exactly one positional argument!",
        ));
    }

    let filename = &args[1];
    let lines: Vec<String> = load_file(filename)?;
    let slopes = vec![
        Slope::new(&lines, 1, 1),
        Slope::new(&lines, 3, 1),
        Slope::new(&lines, 5, 1),
        Slope::new(&lines, 7, 1),
        Slope::new(&lines, 1, 2),
    ];

    let mut product = 1;
    for s in slopes {
        product *= s.filter(|&t| t).count();
    }

    println!("The product is: {}", product);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_data() -> Slope {
        let lines: Vec<String> = vec![
            "..#.......#".to_string(),
            "#..........".to_string(),
            "...........".to_string(),
            "...........".to_string(),
            "...........".to_string(),
            "#..........".to_string(),
            "...........".to_string(),
        ];

        Slope::new(&lines, 0, 0)
    }

    #[test]
    fn slide_test_fit() {
        let mut slope = create_test_data();

        slope.dx = 2;
        assert!(slope.next().unwrap());
        assert_eq!(slope.x, 2);
        assert_eq!(slope.y, 0);

        slope.dx = 8;
        assert!(slope.next().unwrap());
        assert_eq!(slope.x, 10);
        assert_eq!(slope.y, 0);

        slope.dx = 1;
        assert!(!slope.next().unwrap());
        assert_eq!(slope.x, 0);
        assert_eq!(slope.y, 0);

        slope.dx = 11;
        assert!(!slope.next().unwrap());
        assert_eq!(slope.x, 0);
        assert_eq!(slope.y, 0);

        slope.dy = 1;
        slope.dx = 0;
        assert!(slope.next().unwrap());
        assert_eq!(slope.x, 0);
        assert_eq!(slope.y, 1);

        slope.dy = 5;
        assert!(!slope.next().unwrap());
        assert_eq!(slope.x, 0);
        assert_eq!(slope.y, 6);

        slope.dy = 1;
        match slope.next() {
            Some(_) => assert!(false, "We should have finished now!"),
            None => {
                assert_eq!(slope.x, 0);
                assert_eq!(slope.y, 6);
            }
        }

        slope.dy = 0;
        assert!(!slope.next().unwrap());
        assert_eq!(slope.x, 0);
        assert_eq!(slope.y, 6);
    }
}
