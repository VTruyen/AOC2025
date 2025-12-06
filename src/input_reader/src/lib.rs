use std::io::BufRead;

pub struct InputReader {
    pub reader_mod: ReaderMod
}

impl InputReader {
    pub fn read_lines(&self) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>> {
        let file = std::fs::File::open(self.reader_mod.path())?;
        Ok(std::io::BufReader::new(file).lines())
    }
}

pub enum ReaderMod {
    Example,
    Real
}

impl ReaderMod {
    pub fn path(&self) -> &'static str {
        match self {
            ReaderMod::Example => "src/example_input.txt",
            ReaderMod::Real => "src/real_input.txt",
        }
    }
}
