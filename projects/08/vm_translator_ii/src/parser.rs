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

use std::path::Path;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::u32;
use std::fmt;

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
   command_type: CommandType,
   file_name:    String     // This is needed since static variables need to be xxx.j where xxx is the name of the vm file
}

pub struct Parser {
   current_command:     Option<Command>,
   reader:              BufReader<File>,
   has_more_commands:   bool,
   file_name:           String
}


impl Parser {
   pub fn new(input_file: String) -> Parser {
      let file = File::open(&input_file).expect("Error opening input file.");

      let path = Path::new(&input_file);

      let file_name = if let Some(filename) = path.file_name() {
         if let Some(filename_str) = filename.to_str() {
            filename_str
         } else {
            translation_error("Couldn't deduce filename from input file.")
         }
      } else {
         translation_error("Couldn't deduce filename from input file.")
      };

      Parser {
         current_command: None,
         reader: BufReader::new(file),
         has_more_commands: true,
         file_name: file_name.to_string()
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
         *c
      } else {
         translation_error(&format!("Invalid command: {}", command));
      };

      // --- Arithmetic ---
      if ARITH_COMMANDS.contains(&c) {
         self.current_command = Some(
            Command {
               arg1: c.to_string(),
               arg2: None,
               command_type: CommandType::Arithmetic,
               file_name: self.file_name.clone()
            }
         );
         return
      }

      match c {
         "push" | "pop" => {
               let segment = self.get_string_from_parts(1, &parts, "push/pop command requires a memory segment.");
               let index   = self.get_string_from_parts(2, &parts, "push/pop command requires an index.");

               self.parse_push_pop(
                  if c == "push" {CommandType::Push} else {CommandType::Pop},
                  segment,
                  index
               )
         },
         "label" => {
            let label_name = self.get_string_from_parts(1, &parts, "'label' command requires a name.");

            self.current_command = Some(
               Command {
                  arg1: label_name,
                  arg2: None,
                  command_type: CommandType::Label,
                  file_name: self.file_name.clone()
               }
            )
         },
         "if-goto" => {
            let label_name = self.get_string_from_parts(1, &parts, "'if-goto' command requires a label name.");

            self.current_command = Some(
               Command {
                  arg1: label_name,
                  arg2: None,
                  command_type: CommandType::If,
                  file_name: self.file_name.clone()
               }
            )
         },
         "goto" => {
            let label_name = self.get_string_from_parts(1, &parts, "'goto' command requires a label name.");

            self.current_command = Some(
               Command {
                  arg1: label_name,
                  arg2: None,
                  command_type: CommandType::Goto,
                  file_name: self.file_name.clone()
               }
            )
         },
         "function" => {
            let function_name = self.get_string_from_parts(1, &parts, "'function' command requires a name.");
            let local_vars    = self.get_string_from_parts(2, &parts, "'function' command requires a local variable count.");

            self.current_command = Some(
               Command {
                  arg1: function_name,
                  arg2: Some(local_vars),
                  command_type: CommandType::Function,
                  file_name: self.file_name.clone()
               }
            )
         },
         "call" => {
            let function_name = self.get_string_from_parts(1, &parts, "'call' command requires a function name.");
            let arg_count     = self.get_string_from_parts(2, &parts, "'call' command requires an argument count.");

            self.current_command = Some(
               Command {
                  arg1: function_name,
                  arg2: Some(arg_count),
                  command_type: CommandType::Call,
                  file_name: self.file_name.clone()
               }
            )
         },
         "return" => {
            self.current_command = Some(
               Command {
                  arg1: c.to_string(),  // This should really be None but then that messes errthing up :/
                  arg2: None,
                  command_type: CommandType::Return,
                  file_name: self.file_name.clone()
               }
            )
         },
         _ => translation_error("Error bad command...")
      }
   }

   
   fn get_string_from_parts(&self, index: usize, parts: &[&str], error_msg: &str) -> String {
      // Should call translation error if there's nothing at the index
      if let Some(s) = parts.get(index) {
         s.to_string()
      } else {
         translation_error(error_msg)
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
               command_type: push_pop,
               file_name: self.file_name.clone()
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

   pub fn get_file_name(&self) -> &String {
      &self.file_name
   }
}

impl fmt::Display for Command {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
         f,
         "Arg1: {}, Arg2: {}, Command Type: {}",
         self.arg1,
         self.arg2.as_deref().unwrap_or("None"),
         self.command_type
      )
   }
}

impl fmt::Display for CommandType {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         CommandType::Arithmetic => write!(f, "Arithmetic"),
         CommandType::Call       => write!(f, "Call"),
         CommandType::Function   => write!(f, "Function"),
         CommandType::Goto       => write!(f, "Goto"),
         CommandType::If         => write!(f, "If"),
         CommandType::Label      => write!(f, "Label"),
         CommandType::Pop        => write!(f, "Pop"),
         CommandType::Push       => write!(f, "Push"),
         CommandType::Return     => write!(f, "Return")
      }
   }
}