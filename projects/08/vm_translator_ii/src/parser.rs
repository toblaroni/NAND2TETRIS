/* ==========================================================================
 *
 *   Parser:
 * - Handles the parsing of a single .vm file
 * - Reads a VM command, parses the command into its lexical components, 
 *   and provides convenient access to these components
 * - Ignores whitespace and comments
 * 
 * ========================================================================== */

#![allow(dead_code)]

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::u32;

use crate::vm_translator::translation_error;

const ARITH_COMMANDS: &[&str] = &[
   "add", "sub", "neg", 
   "eq",  "gt",  "lt",
   "and", "or",  "not"
];

const MEM_SEGMENTS: &[&str] = &[
   "argument", "local",
   "static",   "constant",
   "this",     "that",
   "pointer",  "temp"
];

#[derive(Clone, Copy)]
pub enum CommandType {
   Arithmetic,
   Call,
   Function,
   Goto,
   If,
   Label,
   Pop,
   Push,
   Return 
}

pub struct Command {
   arg1:         String,
   arg2:         Option<String>,
   command_type: CommandType
}

pub struct Parser {
   current_command:     Option<Command>,
   reader:              BufReader<File>,
   has_more_commands:   bool,
   arithmetic_commands: [&str; 9],
   mem_segments:        [&str; 8]
}


impl Parser {
   pub fn new(input_file: String) -> Parser {
      let file = File::open(input_file).expect("Error opening input file.");

      Parser {
         current_command: None,
         reader: BufReader::new(file),
         has_more_commands: true,
         arithmetic_commands: [
            "add", "sub", "neg", 
            "eq",  "gt",  "lt",
            "and", "or",  "not"
         ],
         mem_segments: [
            "argument", "local",
            "static",   "constant",
            "this",     "that",
            "pointer",  "temp"
         ]
      }
   }

   pub fn advance(&mut self) {
      // ======================================================
      // Reads the next command from the input and makes it the
      // *current command*.
      // Only called if there's more commands.
      // Initially there is not current command.
      // ======================================================

      // Set current command to empty to avoid weird bugs
      // Not really necessary since if we're advancing correctly current_command will always be udpated
      self.current_command = None;
      let mut line = String::new();

      match self.reader.read_line(&mut line) {
         Ok(0) => {
            // EOF
            self.has_more_commands = false;
         },
         Ok(_) => {
            if self.is_comment(&line) || line.trim().is_empty() {    // Eat comments and whitespace
               self.advance()
            } else {
               // Now we've found the next command, we need to update the 'current_command' field
               // Remove inline comments and white space
               self.remove_inline_comment(&mut line);
               self.parse_command(line)
            }
         },
         Err(_) => {
            translation_error("Error occurred while reading source file.")
         }
      };
   }
   

   fn remove_inline_comment(&self, line: &mut String ) {
      if let Some(index) = line.find("//") {
         *line = line[..index].trim().to_string()
      } else {
         *line = line[..].trim().to_string()
      };
   }


   fn parse_command(&mut self, command: String) {
      // Build command from current line
      // this will probs have to be re-written for the next project
      let parts: Vec<&str> = command.split(' ').collect();

      // Get the first word in the command
      let c = if let Some(c) = parts.first() {
         c.to_string()
      } else {
         translation_error(&format!("Invalid command: {}", command));
      };
      
      match parts.len() {
         1 => {

            if !self.arithmetic_commands.contains(&c) {
               translation_error(&format!("Invalid command: {}", command));
            }

            self.current_command = Some(Command {
               arg1: c,
               arg2: None,
               command_type: CommandType::Arithmetic
            })
         }
         3 => {
            if c == "push" || c == "pop" {
               let segment: String = parts.get(1).unwrap().to_string();
               let index:   String = parts.get(2).unwrap().to_string();

               self.parse_push_pop(
                  if c == "push" {CommandType::Push} else {CommandType::Pop},
                  segment, 
                  index
               );
            }
         },
         _ => translation_error(&format!("Invalid command: {}", command))
      }

   }

   fn parse_push_pop(&mut self, push_pop: CommandType, segment: String, index: String) {
      if !MEM_SEGMENTS.contains(&segment.as_str()) {
         translation_error("Invalid memory segment")
      }

      // Check that the index can be parsed as u32 otherwise it's invalid
      match index.parse::<u32>() {
         Ok(_) => {
            self.current_command = Some(Command {
               arg1: segment,
               arg2: Some(index),
               command_type: push_pop
            })
         },
         Err(_) => translation_error(&format!("Invalid index for push/pop command: {}", index))
      };

   }


   pub fn get_current_command(&self) -> Option<&Command> {
      self.current_command.as_ref()
   }


   fn is_comment(&self, line: &str) -> bool {
      line.trim().starts_with("//")
   }

   pub fn has_more_commands(&self) -> bool {
      self.has_more_commands
   }
}


impl Command {
   pub fn get_arg1(&self) -> &String {
      /*
      * Returns the first argument of the current command.
      * In the case of C_ARITHMETIC, the command itself (add, sub, ...) is returned.
      * Shouldn't be called if the current command is C_RETURN.
      */
      &self.arg1
   }

   pub fn get_arg2(&self) -> Option<&String> {
     /*
      * Returns the second argument of the current command.
      * Only called if the current command is C_PUSH, C_POP, C_FUNCTION or C_CALL.
      */
      self.arg2.as_ref()
   }

   pub fn get_command_type(&self) -> &CommandType {
      &self.command_type
   }
}