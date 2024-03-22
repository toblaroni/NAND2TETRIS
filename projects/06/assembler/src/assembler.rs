use std::fs::File;
use std::io::{BufRead, BufReader, Write, Error};

use crate::parser::{parse_line, Instruction};
use crate::translator::translate_instruction;
use crate::symbol_table::first_pass;

pub fn assemble(source_file: &String) -> Result<(), Error> {
    let file = File::open(source_file)?;

    let output_file = if let Some(index) = source_file.rfind('.') {
        source_file[..index].to_owned() + ".hack"
    } else {
        source_file.to_owned() + ".hack"
    };

    let reader = BufReader::new(file);

    let sym_table = first_pass(&reader);
    second_pass(reader, &output_file)?;

    println!("Finished assembling: {} -> {}", source_file, output_file);
    Ok(())
}

fn second_pass(file_reader: BufReader<File>,
               output_file: &String) -> Result<(), Error> {
    /*
     *  Go through the entire program again, parse and translate the program.
     *  Each time a symbolic A-instruction is encountered (@xxx) where xxx is a symbol
     *  not a number, look up xxx in the symbol table.
     *  If the symbol is found in the symbol table then replace it with it's numerical value.
     *  If it's not found, it must represent a new variable. Add the pair (xxx, n) to the 
     *  symbol table, where n is the next available RAM address. The allocated RAM addresses
     *  are consecutive numbers, starting at 16.
     */
    let mut file_writer = File::create(output_file)?;

    for line in file_reader.lines() {
        let line = line?;
        let ins: Instruction = parse_line(line);

        if ins.line_type.is_none() {
            continue;
        }

        let code: String = translate_instruction(ins);
        write_instruction(&mut file_writer, &code)?;
    }

    Ok(())
}

fn write_instruction(file_writer: &mut impl Write, // Must impl write
                     code: &String) -> Result<(), std::io::Error> {
    file_writer.write_all(code.as_bytes())?;
    file_writer.write_all(b"\n")?;
    Ok(())
}