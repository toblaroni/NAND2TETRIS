/* ==========================================================================
 *
 *   Parser:
 * - Handles the parsing of a single .vm file
 * - Reads a VM command, parses the command into its lexical components, 
 *   and provides convenient access to these components
 * - Ignores whitespace and comments
 * 
 * ========================================================================== */

use std::io::{BufRead, BufReader};
use std::fs::File;

enum CommandType {
   CArithmetic,
   CCall,
   CFunction,
   CGoto,
   CIf,
   CLabel,
   CPop,
   CPush,
   CReturn 
}

pub struct Command {
   pub command:      String,
   pub command_type: CommandType
}

pub struct Parser {
   current_command:   Option<Command>,
   reader:            BufReader<File>,
   has_more_commands: bool
}


impl Parser {
   pub fn new(file: File) -> Parser {
      Parser {
         current_command: None,
         reader: BufReader::new(file),
         has_more_commands: true
      }
   }

   pub fn advance(&mut self) {
     /* 
      *  Reads the next command from the input and makes it the
      *  *current command*.
      *  Only called if there's more commands.
      *  Initially there is not current command.
      */ 
      let mut line = String::new();

      match self.reader.read_line(&mut line) {
         Ok(0) => {
            // EOF
            self.has_more_commands = false;
         },
         Ok(_) => {
            if self.is_comment(&line) || line.is_empty() {
               self.advance()
            }

            self.current_command = Some(Command {
               command: line,
               command_type: CommandType::CArithmetic
            });

         },
         Err(_) => {
            panic!("Error occurred while reading source file.")
         }
      };
   }

   fn is_comment(&self, line: &String) -> bool {
      line.trim().starts_with("//")
   }


   fn has_more_commands(&self) -> bool {
      self.has_more_commands
   }


   pub fn get_command_type(&self) -> Option<CommandType> {
     /*
      *  Returns a constant representing the type of the current command.
      *  Types:
      *  C_ARITHMETIC, C_PUSH, C_POP, C_LABEL, C_GOTO, C_IF, C_FUNCTION, C_RETURN, C_CALL
      */
      match self.current_command {
         Some(command) => Some(command.command_type),
         None          => None
      }
   }

   pub fn arg1() -> Option<String> {
      /*
      * Returns the first argument of the current command.
      * In the case of C_ARITHMETIC, the command itself (add, sub, ...) is returned.
      * Shouldn't be called if the current command is C_RETURN.
      */

      None
   }

   pub fn arg2() -> Option<String> {
     /*
      * Returns the second argument of the current command.
      * Only called if the current command is C_PUSH, C_POP, C_FUNCTION or C_CALL.
      */
      None
   }
}
