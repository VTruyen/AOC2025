mod direction;
mod papers;

use crate::papers::{Entity, from_char};
use input_reader::{InputReader, ReaderMod};
use std::fs::File;
use std::io::{BufReader, Lines};

fn main() {
    let reader = InputReader {
        reader_mod: ReaderMod::Real,
    };
    let lines = reader.read_lines().expect("Could not read lines");
    let mut result = 0;
    let matrix = create_matrix(lines);
    matrix.iter().for_each(|line| {
        line.iter()
            .filter(|&entity| matches!(entity.entity_type, papers::EntityType::Paper))
            .for_each(|entity| {
                match entity.check_freedom(&matrix) {
                    true => result += 1,
                    false => { /* do nothing */ }
                }
            });
    });
    println!("Result: {}", result);
}

fn create_matrix(lines: Lines<BufReader<File>>) -> Vec<Vec<Entity>> {
    let mut matrix: Vec<Vec<Entity>> = Vec::new();
    lines
        .enumerate()
        .map(|(idx, line)| {
            line.expect("Could not read line")
                .char_indices()
                .map(|num_str| from_char(num_str.1, num_str.0, idx))
                .collect::<Vec<Entity>>()
        })
        .for_each(|line| matrix.push(line));
    matrix
}
