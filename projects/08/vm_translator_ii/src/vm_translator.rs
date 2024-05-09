
use std::process;

use crate::parser;
use crate::code_writer;

pub fn vm_translate(input_file: String) {
    /* ==================================================================
     *  input  -> fileName.vm
     *  output -> fileName.asm
     *  
     *  Logic
     *  - Constructs parser to handle the input file
     *  - Constructs codeWriter to handle the output file
     *  - Marches through the file, parsing each line and generating code.
     * 
     *  Translator is extremely temperamental and will panic at when 
     *  encountering any bug.
     *  Probs better to handle these with Result<> but idk...
     * 
     * ================================================================== */

    let output_file = if let Some(index) = input_file.rfind(".vm") {
        input_file[..index].to_owned() + ".asm"
    } else {
        // Handle folder??
        panic!("Input file requires .vm extension")
    };

    let mut parser      = parser::Parser::new(input_file);
    let mut code_writer = code_writer::CodeWriter::new(&output_file);
    
    while parser.has_more_commands() {
        parser.advance();  // Update parser.currentCommand
        
        if let Some(command) = parser.get_current_command() {
            println!("{}", command);
            // code_writer.translate_command(command)
        }
    }

    println!("Successfully translated source VM file\nOutput -> {}", output_file);
}

pub fn translation_error(msg: &str) -> ! {
    eprintln!("Error: {}", msg);
    process::exit(-1);
}