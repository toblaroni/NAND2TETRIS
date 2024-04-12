/* ==================================================
 * Generates assembly code from the parsed VM command.
 * ================================================== */

#![allow(dead_code)]

use std::io::BufWriter;
use std::fs::File;
use std::io::Write;

use crate::parser::Command;
use crate::parser::CommandType::*;

pub struct CodeWriter {
    writer: BufWriter<File>
}

impl CodeWriter {
    pub fn new(output_file: String) -> CodeWriter {
        let file = File::create(output_file)
                               .expect("Couldn't open output file");

        CodeWriter {
            writer: BufWriter::new(file)
        }

    }

    pub fn translate_command(&mut self, command: &Command) {
        match command.get_command_type() {
            Arithmetic => self.translate_arithmetic(command),
            Pop | Push => self.translate_push_pop(command),
            _ => println!("Soz, not implemented yet...")
        }
    
    }

    fn translate_arithmetic(&mut self, command: &Command) {
        // Translates arithmetic command
        println!("Translating arithmetic command");
        self.write_string(command.get_arg1())

    }

    fn translate_push_pop(&mut self, command: &Command) {
        // Translates push_pop command
        println!("Translating push / pop command");

    }

    fn write_string(&mut self, string: String) {
        // Take a string and write that shit to the output file
        let string = format!("{}\n", string);
        self.writer.write_all(string.as_bytes())
                   .expect("Error occurred while writing to output.");
    }
}

