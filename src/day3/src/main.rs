mod bank;
use input_reader::{InputReader, ReaderMod};
use crate::bank::Bank;

fn main() {
    let reader = InputReader { reader_mod: ReaderMod::Real };
    let lines = reader.read_lines().expect("Could not read lines");
    let result = lines.map(|line| {
        let line = line.expect("Could not read line");
        Bank { number_str: line }
    }).map(|bank| bank.compute_max_jolt_part2())
        .map(|res| res.parse::<i64>().unwrap_or(0))
        .sum::<i64>();
    println!("Result: {}", result);
}
