
use crate::parser;
use crate::codeWriter;

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
     * ================================================================== */

    let output_file = if let Some(index) = input_file.rfind(".vm") {

    } else {
        panic!("")
    };

    let mut parser      = parser::Parser::new(input_file);
    let mut code_writer = codeWriter::CodeWriter::new(output_file);

    while parser.has_more_commands() {
        parser.advance();  // Update parser.currentCommand

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
