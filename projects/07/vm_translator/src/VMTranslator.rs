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

    let file = File::open(input_file).expect("Error opening input file.");

    let parser = parser::Parser::new(file);
}
