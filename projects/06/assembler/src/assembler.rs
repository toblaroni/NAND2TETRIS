use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::parser::{parse, Instruction};

pub fn assemble(source_file: &String) -> Result<(), std::io::Error> {
    let file = File::open(source_file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let ins: Instruction = parse(line);
        print!("{}", ins);

    }

    Ok(())
}