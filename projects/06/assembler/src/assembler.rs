use std::fs::File;
use std::io::{BufRead, BufReader, Write, Error};

use crate::parser::{parse_line, Instruction, is_label, is_instruction};
use crate::translator::translate_instruction;
use crate::symbol_table::{init_sym_table, SymbolTable, add_symbol};

pub fn assemble(source_file: &String) -> Result<(), Error> {
    let output_file = if let Some(index) = source_file.rfind('.') {
        source_file[..index].to_owned() + ".hack"
    } else {
        source_file.to_owned() + ".hack"
    };

    let mut sym_table = init_sym_table();

    first_pass(source_file, &mut sym_table)?;
    second_pass(source_file, &output_file, &mut sym_table)?;

    println!("Finished assembling: {} -> {}", source_file, output_file);
    Ok(())
}

fn first_pass(source_file: &String, sym_table: &mut SymbolTable) -> Result<(), Error> {
    /*
     *  Does the first pass and builds up the symbol table.
     *  Incrememnt current command whenever a C or A instruction is encountered.
     *  It is not incremented when a label, pseudocommand or a comment is encountered 
     */
    let file = File::open(source_file)?;
    let reader = BufReader::new(file);

    let mut current_command: u32 = 0;
    
    for line in reader.lines() {
        let line = line?;

        if is_label(&line) { 
            // This logic should probs be in parser since we're parsing the label...
            let mut chars = line.chars();
            chars.next();       // Remove first char '('
            chars.next_back();  // Remove last char ')'

            add_symbol(chars.as_str().to_string(), current_command.to_string(), sym_table)
        } else if is_instruction(&line) {
            // println!("Incrementing current command. {}", line);
            current_command += 1;
        }
    }
    
    Ok(())
}

fn second_pass(source_file: &String,
               output_file: &String,
               _sym_table: &mut SymbolTable) -> Result<(), Error> {
    /* -----------------------------------------------------------------------
     *  Go through the entire program again, parse and translate the program.
     *  Each time a symbolic A-instruction is encountered (@xxx) where xxx is a symbol
     *  not a number, look up xxx in the symbol table.
     *  If the symbol is found in the symbol table then replace it with it's numerical value.
     *  If it's not found, it must represent a new variable. Add the pair (xxx, n) to the 
     *  symbol table, where n is the next available RAM address. The allocated RAM addresses
     *  are consecutive numbers, starting at 16.
     * ----------------------------------------------------------------------- */
    let file = File::open(source_file)?;
    let reader = BufReader::new(file);
    let mut file_writer = File::create(output_file)?;

    for line in reader.lines() {
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