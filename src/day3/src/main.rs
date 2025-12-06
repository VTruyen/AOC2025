use input_reader::{InputReader, ReaderMod};

fn main() {
    let reader = InputReader { reader_mod: ReaderMod::Example };
    let lines = reader.read_lines().expect("Could not read lines");
    lines.for_each(|line| println!("{}", line.unwrap()));
}
