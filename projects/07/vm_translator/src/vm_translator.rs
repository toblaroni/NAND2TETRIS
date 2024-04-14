
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
     * ================================================================== */

    let output_file = if let Some(index) = input_file.rfind(".vm") {
        input_file[..index].to_owned() + ".asm"
    } else {
        panic!("Input file requires .vm extension")
    };

    println!("Output file: {}", output_file);

    let mut parser      = parser::Parser::new(input_file);
    let mut code_writer = code_writer::CodeWriter::new(output_file);

    while parser.has_more_commands() {
        parser.advance();  // Update parser.currentCommand

        if let Some(command) = parser.get_current_command() {

            code_writer.translate_command(command)
        }

    }
}
