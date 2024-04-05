/* ==========================================================================
 *
 *   Parser:
 * - Handles the parsing of a single .vm file
 * - Reads a VM command, parses the command into its lexical components, 
 *   and provides convenient access to these components
 * - Ignores whitespace and comments
 * 
 * ========================================================================== */

use std::io::BufReader;
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

}

pub struct Parser {
   pub current_command: Option<Command>,
   pub reader: BufReader<File>
}

impl Parser {
   pub fn new(file: File) -> Parser {
      Parser {
         current_command: None,
         reader: BufReader::new(file)
      }
   }

   pub fn has_more_commands() -> bool {
      false
   }

   pub fn advance() {
     /* 
      *  Reads the next command from the input and makes it the
      *  *current command*.
      *  Only called if there's more commands.
      *  Initially there is not current command.
      */ 

   }
   pub fn command_type() -> Option<CommandType> {
     /*
      *  Returns a constant representing the type of the current command.
      *  
      *  Types:
      *  C_ARITHMETIC, C_PUSH, C_POP, C_LABEL, C_GOTO, C_IF, C_FUNCTION, C_RETURN, C_CALL
      */

      None
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
