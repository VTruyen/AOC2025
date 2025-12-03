use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Direction {
    LEFT,
    RIGHT,
}

fn parse_line(line: &str) -> (Direction, i32) {
    let (dir, rest) = line.split_at(1);

    let direction = Direction::from_str(dir);
    let value = rest.parse().expect("Invalid number");

    (direction, value)
}

impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            _ => panic!("Invalid direction"),
        }
    }

    fn apply(self, value: i32, start: i32) -> i32 {
        match self {
            Direction::LEFT => (100 + start - value % 100)%100,
            Direction::RIGHT => ( start + value )%100,
        }
    }
}

fn main() {
    let mut position: i32 = 50;
    let lines = read_lines("src/real_input1.txt").expect("Failed to read lines");

    let count_hits_zero = lines.fold(0, |acc, line| {
        let line = line.expect("Could not read line");
        let (direction, value) = parse_line(&line);

        position = direction.apply(value, position);

        if position == 0 {
            acc + 1
        } else {
            acc
        }
    });

    println!("Result: {}", count_hits_zero);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}