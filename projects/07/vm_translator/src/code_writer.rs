/* ==================================================
 * Generates assembly code from the parsed VM command.
 * 
 * One optimisation could be to have a command buffer
 * which stores all commands as an array in memory 
 * instead of constantly writing to the output file?
 * ================================================== */

#![allow(non_snake_case)]

use std::io::{BufWriter, Write};
use std::fs::File;

use crate::parser::{Command, CommandType};
use crate::vm_translator::translation_error;


pub struct CodeWriter {
    writer: BufWriter<File>,
    
    comp_count: u32     // No. of 'store true' labels.
}

impl CodeWriter {
    pub fn new(output_file: String) -> CodeWriter {
        let file = File::create(output_file)
                               .expect("Couldn't open output file");

        CodeWriter {
            writer: BufWriter::new(file),
            comp_count: 0,
        }

    }

    pub fn translate_command(&mut self, command: &Command) {
        match command.get_arg2() {
            Some(arg2) => println!("Arg1: {}, Arg2: {}", command.get_arg1(), arg2),
            None => println!("Arg1: {}, Arg2: None", command.get_arg1()),
        }


        match command.get_command_type() {
            CommandType::Arithmetic => self.translate_arithmetic(command),
            CommandType::Push => self.translate_push(command),
            _ => println!("Soz, not implemented yet...")
        }
    
    }

    fn translate_arithmetic(&mut self, command: &Command) {
        // Translates arithmetic command
        println!("Translating arithmetic command");
        
        match command.get_arg1().as_str() {
            "add" => self.two_var_arithmetic("M=M+D"),
            "sub" => self.two_var_arithmetic("M=M-D"),
            "neg" => self.write_strings(&vec!["@SP", "A=M-1", "M=-M"]),
            "eq"  => self.compare_arithmetic("D;JEQ"),
            "gt"  => self.compare_arithmetic("D;JLT"),
            "lt"  => self.compare_arithmetic("D;JGT"),
            "and" => self.two_var_arithmetic("M=D&M"),
            "or"  => self.two_var_arithmetic("M=D|M"),
            "not" => self.write_strings(&vec!["@SP", "A=M-1", "M=!M"]),
            _     => translation_error(&format!("Bad arithmetic command {}", command.get_arg1()))
        };
    }

    fn compare_arithmetic(&mut self, comp: &str) {
        /*
         *  Handles gt, lt and eq. 
         *  We have an IF statement (if x == y store TRUE, else FALSE),
         *  Therefore we need a label and a jump... 
         *  We can use comp_count to write out different labels
         *  this is not the most efficient way...
         *  EXAMPLE (eq)
         *      @SP
         *      A=M-1 
         *      D=M
         *      SP--
         *      @SP
         *      A=M-1
         *      D=D-M      
         *      @true_<comp_count>
         *      D;JEQ   // JEQ will change depending on jmp_type
         *      @SP     
         *      A=M-1
         *      M=0     // Store false
         *      @comp_end_<comp_count>
         *      ;JMP
         *  (true_<comp_count>)
         *      @SP
         *      A=M-1
         *      M=-1
         *  (comp_end_<comp_count>)
         * 
         */

        let true_label     = format!("true_{}", self.comp_count);
        let comp_end_label = format!("comp_end_{}", self.comp_count);

        self.write_strings(&vec!["@SP", "A=M-1", "D=M"]);
        self.modify_SP(false);
        self.write_strings(&vec!["@SP", "A=M-1", "D=D-M"]);
        
        self.write_string(&format!("@{}", true_label));
        self.write_string(comp);
        // Store false
        self.write_strings(&vec!["@SP", "A=M-1", "M=0"]);
        self.write_string(&format!("@{}", comp_end_label));
        self.write_string("0;JMP");

        // Store true
        self.write_string(&format!("({})", true_label));
        self.write_strings(&vec!["@SP", "A=M-1", "M=-1"]);

        self.write_string(&format!("({})", comp_end_label));
        self.comp_count += 1;
    }


    fn two_var_arithmetic(&mut self, arith_command: &str) {
        /*
         *  Most of the arithmetics have the same code:
         *  EXAMPLE (ADD)
         *      @SP
         *      A=M-1
         *      D=M   // Store y in D
         *      SP--
         *      @SP
         *      A=M-1
         *      ------ Everthing above this is generic
         *      M=M+D   <-- arith_command
         */
        self.write_strings(&vec!["@SP", "A=M-1","D=M"]);
        self.modify_SP(false);                                     // SP--
        self.write_strings(&vec!["@SP", "A=M-1", arith_command]);
    }

    fn translate_push(&mut self, command: &Command) {
        // Translates push_pop command
        println!("Translating push command");

        match command.get_arg1().as_str() {
            "argument" => println!("argument not implemented"),
            "local"    => println!("local not implemented"),
            "this"     => println!("this not implemented"),
            "that"     => println!("that not implemented"),
            "static"   => println!("static not implemented"),
            "constant" => self.push_constant(command),
            "pointer"  => println!("pointer not implemented"),
            "temp"     => println!("temp not implemented"),
            _          => translation_error(&format!("Invalid memory location: {}", command.get_arg1()))
        };
    }

    fn push_constant(&mut self, command: &Command) {
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
        self.write_strings(&vec![&label, "D=A"]);

        // Deref SP and inc sp
        self.deref_SP();
        self.write_string("M=D");

        self.modify_SP(true);
    }


    fn generic_mem_push(&mut self, mem_seg: &str, index: &str) {
        /*  I think most the code for local, argument, this and that will be the same
         *  Code for 'pop' will be similar but in reverse
         *  EXAMPLE (push local <index>) 
         *      @LCL        // This will change depending on the segment (mem_seg)
         *      D=A
         *      @<index>
         *      A=D+A       // M=[LCL+<index>]
         *      D=M
         *      @SP
         *      A=M
         *      M=D
         *      SP++
         */
        let index_label = &format!("@{}", index);
        self.write_strings(&vec![mem_seg, "D=A"]);

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