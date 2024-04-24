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
            CommandType::Push       => self.translate_push(command),
            CommandType::Pop        => self.translate_pop(command),
            _                       => println!("Soz, not implemented yet...")
        }
    
    }

    fn translate_arithmetic(&mut self, command: &Command) {
        println!("Translating arithmetic command");
        
        match command.get_arg1().as_str() {
            "add" => self.two_var_arithmetic("M=M+D"),
            "sub" => self.two_var_arithmetic("M=M-D"),
            "neg" => self.write_strings(&["@SP", "A=M-1", "M=-M"]),
            "eq"  => self.compare_arithmetic("D;JEQ"),
            "gt"  => self.compare_arithmetic("D;JLT"),
            "lt"  => self.compare_arithmetic("D;JGT"),
            "and" => self.two_var_arithmetic("M=D&M"),
            "or"  => self.two_var_arithmetic("M=D|M"),
            "not" => self.write_strings(&["@SP", "A=M-1", "M=!M"]),
            _     => translation_error(&format!("Bad arithmetic command {}", command.get_arg1()))
        };
    }

    fn translate_push(&mut self, command: &Command) {
        println!("Translating push command");

        let index = if let Some(i) = command.get_arg2() {
            i
        } else {
            translation_error(&format!("No index was given: push {}", command.get_arg1()))
        };

        match command.get_arg1().as_str() {
            "argument" => self.generic_mem_push("@ARG", index),
            "local"    => self.generic_mem_push("@LCL", index),
            "this"     => self.generic_mem_push("@THIS", index),
            "that"     => self.generic_mem_push("@THAT", index),
            "static"   => println!("static not implemented"),
            "constant" => self.push_constant(command),
            "pointer"  => println!("pointer not implemented"),
            "temp"     => self.push_temp(index),
            _          => translation_error(&format!("Invalid memory location: {}", command.get_arg1()))
        };
    }

    fn translate_pop(&mut self, command: &Command) {
        println!("Translating pop command");

        let index = if let Some(i) = command.get_arg2() {
            i
        } else {
            translation_error(
                &format!("No index was given: pop {}", command.get_arg1())
            )
        };

        match command.get_arg1().as_str() {
            "argument" => self.generic_mem_pop("@ARG", index),
            "local"    => self.generic_mem_pop("@LCL", index),
            "this"     => self.generic_mem_pop("@THIS", index),
            "that"     => self.generic_mem_pop("@THAT", index),
            "static"   => println!("static not implemented"),
            "constant" => translation_error("Can't pop to 'constant' memory segment."),
            "pointer"  => println!("pointer not implemented"),
            "temp"     => self.pop_temp(index),
            _          => translation_error(&format!("Invalid memory location: {}", command.get_arg1()))
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

        self.write_strings(&["@SP", "A=M-1", "D=M"]);
        self.modify_SP(false);
        self.write_strings(&["@SP", "A=M-1", "D=D-M"]);
        
        self.write_string(&format!("@{}", true_label));
        self.write_string(comp);
        // Store false
        self.write_strings(&["@SP", "A=M-1", "M=0"]);
        self.write_string(&format!("@{}", comp_end_label));
        self.write_string("0;JMP");

        // Store true
        self.write_string(&format!("({})", true_label));
        self.write_strings(&["@SP", "A=M-1", "M=-1"]);

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
        self.write_strings(&[
            "@SP",
            "A=M-1",
            "D=M"
        ]);
        self.modify_SP(false);                                     // SP--
        self.write_strings(&[
            "@SP",
            "A=M-1",
            arith_command
        ]);
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
            c   // Assume that c is an unsigned integer... within the correct range
        } else {
            translation_error("Push/Pop command requires a second argument.\n'push constant <constant>")
        };

        // Store the constant in D
        let label = format!("@{}", constant);
        self.write_strings(&[&label, "D=A"]);

        // Deref SP and inc sp
        self.deref_SP();
        self.write_string("M=D");

        self.modify_SP(true);
    }

    fn generic_mem_push(&mut self, mem_seg: &str, index: &str) {
        /*  
         *  
         *  EXAMPLE (push local <index>) 
         *      @LCL        // This will change depending on the segment (mem_seg)
         *      D=M
         *      @<index>
         *      A=D+A       // M=[LCL+<index>]
         *      D=M
         *      @SP
         *      A=M
         *      M=D
         *      SP++
         */
        let index_label = &format!("@{}", index);
        self.write_strings(&[
            mem_seg,
            "D=M",
            index_label,
            "A=D+A",        // M=RAM[*LCL+<index>]
            "D=M",
            "@SP",
            "A=M",
            "M=D"
        ]);
        self.modify_SP(true);
    }


    fn push_temp(&mut self, index: &str) {
        /*  Temp = RAM[5-12]
         *  Assume that index is in the right range...
         *  
         *      @<index>
         *      D=A
         *      @5      // Base of temp
         *      A=D+A   // M = RAM[index+5]
         *      D=M
         *      @SP
         *      A=M-1
         *      M=D
         *      SP++
         */
        let index_label = &format!("@{}", index);
        self.write_strings(&[
            index_label,
            "D=A",
            "@5",       // Base of temp
            "A=D+A",    // M = RAM[index+5]
            "D=M",
            "@SP",
            "A=M",
            "M=D"
        ]);
        self.modify_SP(true)
    }

    fn generic_mem_pop(&mut self, mem_seg: &str, index: &str) {
        /*
         *  It's not where ya binnnnn,
         *  but where you gonna goooooooooo...
         *  
         *  - Store the value of mem_seg + index in R13
         *  - Grab the pop top of the stack to D
         *  - Store D in Ram[R13]
         * 
         *      @<index> 
         *      D=A
         *      @<mem_seg>      // Since mem_seg is a pointer, we want the value at RAM[mem_seg]
         *      D=M+D           
         *      @R13
         *      M=D        // Store mem_seg + index in R13
         *      @SP
         *      A=M-1
         *      D=M
         *      SP --
         *      @R13
         *      A=M
         *      M=D
         *      
         */

        // Store mem_seg + index in R13
        let index_label   = &format!("@{}", index);
        
        self.write_strings(&[
            index_label,
            "D=A",
            mem_seg,
            "D=M+D",            // D=RAM[<mem_seg>]+index
            "@R13",
            "M=D",              // store mem_seg in R13
            "@SP",
            "A=M-1",
            "D=M"
        ]);
        self.modify_SP(false);  
        self.write_strings(&[
            "@R13",
            "A=M",
            "M=D"
        ]);
    }

    fn pop_temp(&mut self, index: &str) {
        // Similar to generic_pop but we don't have to deref mem_seg
        let index_label = &format!("@{}", index);
        self.write_strings(&[
            index_label,
            "D=A",
            "@5",
            "D=D+A",
            "@R13",
            "M=D",
            "@SP",
            "A=M-1",
            "D=M"
        ]);
        self.modify_SP(false);
        self.write_strings(&[
            "@R13",
            "A=M",
            "M=D"
        ])
    }


    fn deref_SP(&mut self) {                // TODO Remove this function
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