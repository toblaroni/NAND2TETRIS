/* ==================================================
 * Generates assembly code from the parsed VM command.
 * ================================================== */

use std::io::BufWriter;
use std::fs::File;

use crate::parser::{Command, CommandType};

pub struct CodeWriter {
    writer: BufWriter<File>
}

impl CodeWriter {
    pub fn new(output_file: String) -> CodeWriter {
        let file: File = File::open(output_file).expect("Couldn't open output file");

        CodeWriter {
            writer: BufWriter::new(file)
        }

    }

    pub fn translate_command(command: Option<Command>) {
        match command {
            Some(command) => {
                if command.get_command_type
            },
            None => return
        }
    
    }

    fn translate_arithmetic(command: Command) {
        // Translates arithmetic command

    }

    fn translate_push_pop(command: Command) {
        // Translates push_pop command

    }

    fn write_arithmetic(comand: String) {
        // Translates an arithmetic command to the output
    }

    fn write_push_pop(push_pop: String, segment: String, index: i32) { 
        // Translates a push or pop command to the output

    }
}

