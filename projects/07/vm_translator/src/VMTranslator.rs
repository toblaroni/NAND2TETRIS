use std::fs::File;

use crate::parser;

pub fn vm_translate(input_file: String) {
    /*
     *  input  -> fileName.vm
     *  output -> fileName.asm
     *  
     *  Logic
     *  - Constructs parser to handle the input file
     *  - Constructs codeWriter to handle the output file
     *  - Marches through the file, parsing each line and generating code.
     */

    let file = File::open(input_file)
                    .expect("Error opening input file.");

    let mut parser = parser::Parser::new(file);
    while parser.has_more_commands() {
        parser.advance();  // Advance to the next command

        if let Some(arg1) = parser.arg1() {
            if let Some(arg2) = parser.arg2() {
                println!("arg1: {}, arg2: {}", arg1, arg2)
            } else {
                println!("arg1 {}", arg1)
            }
        }

        // Translate parser.current_command
    }
}
