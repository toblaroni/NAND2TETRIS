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
use std::path::Path;

use crate::parser::{Command, CommandType};
use crate::vm_translator::translation_error;


pub struct CodeWriter {
    writer: BufWriter<File>,
    comp_count: u32,             // No. of 'store true' labels.
    file_name: String            // Name of the file without the extension
}

impl CodeWriter {
    pub fn new(output_file: &String) -> CodeWriter {
        let file = File::create(output_file)
                               .expect("Couldn't open output file");

        let path = Path::new(output_file);
        let file_name = path.file_stem()
                            .and_then(|f| f.to_str())
                            .unwrap_or_default();

        CodeWriter {
            writer: BufWriter::new(file),
            comp_count: 0,
            file_name: file_name.to_string()
        }

    }


    pub fn init(&mut self) {
        // SP = 256
        // call Sys.init
        self.write_strings(&[
            "@SP",
            "M=256"
        ]);
    }


    pub fn translate_command(&mut self, command: &Command) {
        match command.get_command_type() {
            CommandType::Arithmetic => self.translate_arithmetic(command),
            CommandType::Push       => self.translate_push(command),
            CommandType::Pop        => self.translate_pop(command),
            CommandType::Call       => self.translate_call(command),
            CommandType::Function   => self.translate_function(command),
            CommandType::Goto       => self.translate_goto(command),
            CommandType::If         => self.translate_if(command)
            CommandType::Label      => self.translate_label(command)
            CommandType::Return     => self.translate_return(command)
        }
    
    }


    fn translate_arithmetic(&mut self, command: &Command) {
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


    fn translate_label(&mut self, command: &Command) {

    }


    fn translate_goto(&mut self, command: &Command) {

    }


    fn translate_if(&mut self, command: &Command) {

    }


    fn translate_call(&mut self, command: &Command) {

    }

    fn translate_return(&mut self, command: &Command) {

    }


    fn translate_function(&mut self, command: &Command) {

    }


    fn translate_push(&mut self, command: &Command) {
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
            "static"   => self.push_static(index),
            "constant" => self.push_constant(command),
            "pointer"  => self.push_base_index(index, "@3"),
            "temp"     => self.push_base_index(index, "@5"),
            _          => translation_error(&format!("Invalid memory location: {}", command.get_arg1()))
        };
    }


    fn translate_pop(&mut self, command: &Command) {
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
            "static"   => self.pop_static(index),
            "constant" => translation_error("Can't pop to 'constant' memory segment."),
            "pointer"  => self.pop_base_index(index, "@3"),
            "temp"     => self.pop_base_index(index, "@5"),
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
        self.write_strings(&[
            "@SP",
            "A=M-1",
            "D=M"       // Store y in D
        ]);
        self.modify_SP(false);
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
        let label = &format!("@{}", constant);
        self.write_strings(&[
            label,
            "D=A",
            "@SP",
            "A=M",
            "M=D"
        ]);
        self.modify_SP(true);
    }


    fn push_static(&mut self, index: &str) {
        /*
         *  push static <index>
         *  output:
         *      @<file_name>.<index>
         *      D=M
         *      ...Push D to stack...
         *  
         */
        let var_label = &format!("@{}.{}", self.file_name, index);
        self.write_strings(&[
            var_label,
            "D=M",
            "@SP",
            "A=M",
            "M=D"
        ]);
        self.modify_SP(true);
    }


    fn generic_mem_push(&mut self, mem_seg: &str, index: &str) {
        let index_label = &format!("@{}", index);
        self.write_strings(&[
            mem_seg,        // @LCL for example
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


    fn push_base_index(&mut self, index: &str, base: &str) {
        /*
         *  Temp = RAM[5-12], pointer=RAM[3-4]
         *  Assume that index is in the right range...
         * 
         */

        let index_label = &format!("@{}", index);

        self.write_strings(&[
            index_label,
            "D=A",
            base,       // Base of temp/ptr (5/3)
            "A=D+A",    // M = RAM[index+base]
            "D=M",
            "@SP",
            "A=M",
            "M=D"
        ]);
        self.modify_SP(true)
    }


    fn pop_static(&mut self, index: &str) {
        /*
         *  pop static index
         *  Output:
         *      ...Store the top of the stack in D...
         *      @<file_name>.<index>
         *      M=D
         */
        let var_label = &format!("@{}.{}", self.file_name, index);

        self.write_strings(&[
            "@SP",
            "A=M-1",
            "D=M"
        ]);
        self.modify_SP(false);
        self.write_strings(&[
            var_label,
            "M=D"
        ]);
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
         */

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


    fn pop_base_index(&mut self, index: &str, base: &str) {
        let index_label = &format!("@{}", index);
        self.write_strings(&[
            index_label,
            "D=A",
            base,
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