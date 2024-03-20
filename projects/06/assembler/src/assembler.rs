use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::parser::{parse_line, Instruction};
use crate::translator::translate_instruction;

pub fn assemble(source_file: &String) -> Result<(), std::io::Error> {
    let file = File::open(source_file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let ins: Instruction = parse_line(line);

        if let None = ins.line_type {
            continue;
        }
        println!("{}", ins);
        let code: String = translate_instruction(ins);
        println!("{}\n", code);
    }

    Ok(())
}