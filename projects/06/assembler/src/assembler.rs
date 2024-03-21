use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crate::parser::{parse_line, Instruction};
use crate::translator::translate_instruction;

pub fn assemble(source_file: &String) -> Result<(), std::io::Error> {
    let file = File::open(source_file)?;

    let output_file = if let Some(index) = source_file.rfind('.') {
        source_file[..index].to_owned() + ".hack"
    } else {
        source_file.to_owned() + ".hack"
    };

    let reader = BufReader::new(file);
    let mut writer = File::create(&output_file)?;

    for line in reader.lines() {
        let line = line?;
        let ins: Instruction = parse_line(line);

        if ins.line_type.is_none() {
            continue;
        }

        let code: String = translate_instruction(ins);
        write_instruction(&mut writer, &code)?;
    }

    println!("Finished assembling: {} -> {}", source_file, output_file);
    Ok(())
}

fn write_instruction(file_writer: &mut impl Write, // Must impl write
                     code: &String) -> Result<(), std::io::Error> {
    file_writer.write_all(code.as_bytes())?;
    file_writer.write_all(b"\n")?;
    Ok(())
}