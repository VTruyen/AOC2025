use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Direction {
    LEFT,
    RIGHT,
}

fn main() {
    let mut starting_point:i32 = 50;
    let counter = 0;
    let lines = read_lines("src/real_input1.txt")
        .expect("Failed to read lines");
    let result: i32 = lines.fold(counter, |acc, line| {
        if line.is_err() {
            panic!("Could not read line from file");
        }
        let line = line.unwrap();
        let (direction, value)  = line.split_at(1);
        let direction = match direction {
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            _ => panic!("Invalid direction"),
        };
        let value: i32 = value.parse().unwrap();
        starting_point = compute_value_for_direction(direction, value, starting_point);
        if starting_point == 0 { acc + 1 } else { acc }
    });
    println!("Result: {}", result);
}

fn compute_value_for_direction(direction: Direction, value: i32, acc: i32) -> i32 {
    match direction {
        Direction::LEFT => compute_negative_value(value, acc)%100,
        Direction::RIGHT => ( acc + value )%100,
    }
}

fn compute_negative_value(value: i32, acc: i32) -> i32 {
    100 + acc - value % 100
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}