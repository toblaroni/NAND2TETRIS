/* ==================================================
 * Generates assembly code from the parsed VM command.
 * ================================================== */

#![allow(non_snake_case)]

use std::io::{BufWriter, Write};
use std::fs::File;
use std::path::Path;

use crate::parser::{Command, CommandType};
use crate::vm_translator::translation_error;


pub struct CodeWriter {
    writer: BufWriter<File>,
    store_true_count: u32,       // No. of store true labels.
    return_count: u32,           // No. of return address labels.
    func_count: u32,             // No. of function init loop labels.
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
            store_true_count: 0,
            return_count: 0,
            func_count: 0,
            file_name: file_name.to_string()
        }

    }


    pub fn init(&mut self) {
        // Do I need to initialise ARG, LCL, THIS THAT?
        self.write_strings(&[
            "@SP",
            "M=256" // SP = 256
        ]);

        // call Sys.init
    }


    pub fn translate_command(&mut self, command: &Command) {
        match command.get_command_type() {
            CommandType::Arithmetic => self.translate_arithmetic(command),
            CommandType::Push       => self.translate_push(command),
            CommandType::Pop        => self.translate_pop(command),
            CommandType::Call       => self.translate_call(command),
            CommandType::Function   => self.translate_function(command),
            CommandType::Goto       => self.translate_goto(command),
            CommandType::If         => self.translate_if(command),
            CommandType::Label      => self.translate_label(command),
            CommandType::Return     => self.translate_return()
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


    fn translate_call(&mut self, command: &Command) {
        // call <function_name> nArgs

        // 1. push return address

        // <vm_file>.ret<ret_count>
        let ret_addr = format!("{}.ret{}", self.file_name, self.return_count.to_string());
        
        self.write_strings(&[
            &format!("@{}", ret_addr),
            "D=A"
        ]);
        self.push_reg("D");

        // 2. Save the frame of the caller
        //      -> push LCL, ARG, THIS, THAT
        self.write_strings(&[ "@LCL", "D=M"]);
        self.push_reg("D");

        self.write_strings(&[ "@ARG", "D=M"]);
        self.push_reg("D");

        self.write_strings(&[ "@THIS", "D=M"]);
        self.push_reg("D");

        self.write_strings(&[ "@THAT", "D=M"]);
        self.push_reg("D");

        // 3. Reposition ARG for the callee
        //      -> ARG = SP-5-nArgs 
        let nArg_str = if let Some(nArg) = command.get_arg2() {
            nArg
        } else {
            translation_error("Invalid call command. Usage 'call <function_name> <nArgs>'.");
        };

        self.write_strings(&[
            "@5",
            "D=A",
            &format!("@{}", nArg_str),
            "D=D-A",     // D = 5 - nArgs     
            "@SP",
            "D=M-D",    // D = SP - (5-nArgs)
            "@ARG",
            "M=D"
        ]);

        // 4. Set LCL = SP ready for the function to initialise its local variables
        self.write_strings(&[
            "@SP",
            "D=M",
            "@LCL",
            "M=D"
        ]);

        // 5. goto <function_name>
        self.write_strings(&[
            &format!("@{}", command.get_arg1()),    // @<function_name>
            "0;JMP" 
        ]);

        // 6. Make the return address label, this is where the callee can return to... !?
        //      -> (<ret_addr>)
        self.write_string(&format!("({})", ret_addr));
        self.return_count += 1;
    }


    fn push_reg(&mut self, reg: &str) {
        self.write_strings(&[
            "@SP",
            "A=M",
            &format!("M={}", reg)
        ]);
        self.modify_SP(true);
    }


    fn translate_function(&mut self, command: &Command) {
        // function <function_name> nVars

        // 1. Make the function label
        //      -> (<function_name>)
        self.write_string(&format!("({})", command.get_arg1()));


        // 2. Initialise the local vars to zero
        //      -> for nVars: push 0

        let loop_start_label = format!("func_init.loop_start.{}", self.func_count);
        let loop_end_label   = format!("func_init.loop_end.{}", self.func_count);

        let nVars = if let Some(s) = command.get_arg2() {
            s
        } else {
            translation_error("Invalid function command. Usage: function <function_name> <nVars>")
        };

        self.write_strings(&[
            &format!("@{}", nVars), // Init nVars variable and the iterator i
            "D=A",
            "@nVars",
            "M=D",
            "@i",
            "M=0",

            &format!("({})", loop_start_label),
            "@nVars",
            "D=M",
            "@i",
            "D=D-M",    // D = nVars - i
            &format!("@{}", loop_end_label),
            "D;JEQ",    // If nVars - i == 0 finish loop

            "@SP",      // else push 0
            "A=M",
            "M=0",      
            "@SP",
            "M=M+1",

            "@i",
            "M=M+1",    // i++

            &format!("@{}", loop_start_label),
            "0;JMP",

            &format!("({})", loop_end_label)
        ]);

        self.func_count += 1;
    }

    
    fn translate_return(&mut self) {
        // We assume that the return value is at the top of the stack

        self.write_strings(&[
            // 1. Set temp variable FRAME = LCL
            "@LCL",
            "D=M",
            "@FRAME",
            "M=D",

            // 2. Put the return address in a local var
            //      -> RET = *(FRAME-5)
            "@FRAME",
            "D=M",
            "@5",
            "D=D-A",    // D=FRAME-5
            
            "A=D",      // M=RAM[FRAME-5]
            "D=M",      // D=RAM[FRAME-5]
            "@RET",     
            "M=D",      // RET=D

            // 3. Reposition the return value for the caller. Put the return value where ARG is.
            //      -> *ARG = pop()
            "@SP",
            "A=M-1",
            "D=M",      // D = pop()
            "@SP",
            "M=M-1",

            "@ARG",
            "A=M",      // M=RAM[ARG]
            "M=D",

            // 4. Restore SP of the caller
            //      -> SP = ARG + 1
            "@ARG",
            "D=M+1",    // D=ARG+1
            "@SP",
            "M=D",

            // 5. Restore THAT, THIS, ARG and LCL of the caller
            // THAT = *(FRAME-1)
            "@FRAME",
            "M=M-1",        // FRAME -= 1
            "A=M",
            "D=M",
            "@THAT",
            "M=D",

            "@FRAME",
            "M=M-1",        // FRAME =- 1
            "A=M",
            "D=M",
            "@THIS",
            "M=D",

            "@FRAME",
            "M=M-1",        // FRAME =- 1
            "A=M",
            "D=M",
            "@ARG",
            "M=D",
            
            "@FRAME",
            "M=M-1",        // FRAME =- 1
            "A=M",
            "D=M",
            "@LCL",
            "M=D",      // LCL = *(FRAME)

            // 6. goto RET
            "@RET",
            "A=M",
            "0;JMP"
        ]);
    }


    fn translate_label(&mut self, command: &Command) {
        // label <label_name>
        self.write_string(
            &format!("({})", command.get_arg1())
        );
    }


    fn translate_goto(&mut self, command: &Command) {
        // goto <label_name>
        self.write_strings(&[
            &format!("@{}", command.get_arg1()),
            "0;JMP"
        ]);
    }


    fn translate_if(&mut self, command: &Command) {
        // if-goto <label_name>
        self.write_strings(&[
            "@SP",
            "A=M-1",
            "D=M",
            "@SP",
            "M=M-1",
            &format!("@{}", command.get_arg1()),
            "D;JEQ"
        ])

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
         *  We can use store_true_count to write out different labels
         *  this is not the most efficient way...
         *  EXAMPLE (eq)
         *      @SP
         *      A=M-1 
         *      D=M
         *      SP--
         *      @SP
         *      A=M-1
         *      D=D-M      
         *      @true_<store_true_count>
         *      D;JEQ   // JEQ will change depending on jmp_type
         *      @SP     
         *      A=M-1
         *      M=0     // Store false
         *      @comp_end_<store_true_count>
         *      ;JMP
         *  (true_<store_true_count>)
         *      @SP
         *      A=M-1
         *      M=-1
         *  (comp_end_<store_true_count>)
         * 
         */

        let true_label     = format!("true_{}", self.store_true_count);
        let comp_end_label = format!("comp_end_{}", self.store_true_count);

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
        self.store_true_count += 1;
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
