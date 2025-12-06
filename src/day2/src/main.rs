use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use crate::interval::{determine_id_compose_of_pattern, determine_id_status, Interval};

mod interval;

fn main() {
    let lines = read_lines("src/example_input.txt").expect("Could not read lines");
    let sum = lines.flat_map(|line| line.ok())
        .flat_map(|line| parse_line(&line))
        .flat_map(|interval: Interval| {
            let mut results = Vec::new();
            for id in interval.start..=interval.end {
                results.push(determine_id_compose_of_pattern(id));
            }
            results
        })
        .filter( |id| !id.is_valid())
        .map(|id| id.value())
        .sum::<i64>();
    println!("Sum of invalid IDs: {}", sum);
}
// 1227775554
// 1227775554
// 4174379265
// 4174379265

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: &str) -> Vec<Interval> {
    line.split(',')
        .map(|inter_str| inter_str.trim().split('-'))
        .map(|mut parts| {
            let start = parts.next().expect("Missing start").parse().expect("Invalid number");
            let end = parts.next().expect("Missing end").parse().expect("Invalid number");
            Interval { start, end }
        }).collect()
}
