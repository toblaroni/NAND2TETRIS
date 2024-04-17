/* ==================================================
 * Generates assembly code from the parsed VM command.
 * ================================================== */

#![allow(non_snake_case)]

use std::io::{BufWriter, Write};
use std::fs::File;

use crate::parser::Command;
use crate::vm_translator::translation_error;
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
        match command.get_arg2() {
            Some(arg2) => println!("Arg1: {}, Arg2: {}", command.get_arg1(), arg2),
            None => println!("Arg1: {}, Arg2: None", command.get_arg1()),
        }


        match command.get_command_type() {
            Arithmetic => self.translate_arithmetic(command),
            Pop | Push => self.translate_push_pop(command),
            _ => println!("Soz, not implemented yet...")
        }
    
    }

    fn translate_arithmetic(&mut self, command: &Command) {
        // Translates arithmetic command
        println!("Translating arithmetic command");
        

    }

    fn translate_push_pop(&mut self, command: &Command) {
        // Translates push_pop command
        println!("Translating push / pop command");

        // Which mem segment are we working with
        match command.get_arg1().as_str() {
            "argument"      => println!("argument not implemented"),
            "local"         => println!("local not implemented"),
            "static"        => println!("static not implemented"),
            "constant"      => self.pushpop_constant(command),
            "this" | "that" => println!("this / that not implemented"),
            "pointer"       => println!("pointer not implemented"),
            "temp"          => println!("temp not implemented"),
            _               => translation_error(&format!("Invalid memory location: {}", command.get_arg1()))
        };
    }

    fn pushpop_constant(&mut self, command: &Command) {
        /*
         *   @<constant>
         *   D=A
         *   @SP
         *   A=M
         *   M=D
         *   SP++
         */
        
        // Parse constant
        let constant = if let Some(c) = command.get_arg2() {
            match str::parse::<u32>(c) {
                Ok(n) => {
                    if n < 32768 {
                        c
                    } else {
                        translation_error(&format!("Max constant exceeded: {}. Constant should be less than 32768.", c));
                    }
                },
                Err(_) => translation_error(&format!("{} is not an unsigned integer.", c))
            }
        } else {
            translation_error("Push/Pop command requires a second argument.\n'push constant <constant>")
        };

        // Store the constant in D
        let label = format!("@{}", constant);
        let mut strings: Vec<&str> = vec![&label];  // @<constant
        strings.push("D=A");                        // D=A

        self.write_strings(&strings);

        // Deref SP and inc sp
        self.deref_SP();
        self.modify_SP(true);
    }

    fn deref_SP(&mut self) {
        // @SP
        // A=M
        let strings = vec!["@SP", "A=M"];
        self.write_strings(&strings);
    }


    fn modify_SP(&mut self, inc: bool) {
        // Incrementing / Decrementing the value of stack pointer
        let mut strings = vec!["@SP"];
        if inc {
            // Inc address in SP
            strings.push("M=M+1");
        } else {
            // Decc address in SP
            strings.push("M=M-1");
        }
        self.write_strings(&strings);
    }

    fn write_string(&mut self, string: &str) {
        // Take a string and write that shit to the output file
        let string = format!("{}\n", string);
        self.writer.write_all(string.as_bytes())
                   .expect("Error occurred while writing to output.");
    }


    fn write_strings(&mut self, strings: &[&str]) {
        // Write every string in strings on a newline
        for string in strings {
            let string = format!("{}\n", string);
            self.writer.write_all(string.as_bytes())
                       .expect("Error occurred while writing to output.");
        }
    }
}

