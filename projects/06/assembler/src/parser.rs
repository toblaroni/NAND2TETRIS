use core::fmt;
use std::process;

use crate::symbol_table::{find_address, SymbolTable};

pub enum LineType {
    CInstruction,
    AInstruction
}

pub struct Instruction {
    pub line_type: Option<LineType>,

    // C-Instruction
    pub dest: Option<String>,  
    pub comp: Option<String>,  // Shouldn't be optional since comp is always present
    pub jump: Option<String>, 

    // a-Instruction
    pub value: Option<u32>
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        if let Some(LineType::AInstruction) = &self.line_type {
            writeln!(f, "Type = A-Instruction")?
        } else if let Some(LineType::CInstruction) = &self.line_type {
            writeln!(f, "Type = C-Instruction")?
        }

        match self.dest.as_deref() {
            Some(dest) => writeln!(f, "DEST = {}", dest)?,
            None       => writeln!(f, "DEST = None")?
        };
        match self.comp.as_deref() {
            Some(comp) => writeln!(f, "COMP = {}", comp)?,
            None       => writeln!(f, "COMP = None")?
        };
        match self.jump.as_deref() {
            Some(jump) => writeln!(f, "JUMP = {}", jump)?,
            None       => writeln!(f, "JUMP = None")? 
        };
        if let Some(value) = &self.value {
            writeln!(f, "VALUE = {}", value)?
        }

        Ok(())
    }
}

fn build_instruction(line_type: Option<LineType>,
                     dest: Option<String>,
                     comp: Option<String>,
                     jump: Option<String>,
                     value: Option<u32>) -> Instruction {
    Instruction {
        line_type,
        dest,
        comp,
        jump,
        value,
    }
}

fn get_a_instruction(line: String, sym_table: &mut SymbolTable) -> Instruction {

    let value = line[1..].trim();

    let value = match value.parse() {
        Ok(val) => val,
        Err(_)    => {
            // Value is not a number. Look for it in the symbol table
            match find_address(value.to_string(), sym_table) {
                Some(address) => {
                    address.parse().expect("Error occurred while parsing address form symbol table.")
                },
                None => {
                    // If the value is not found in the symbol table. We have a variable.
                    let var_address = sym_table.variable_count + 16;  // Variables start from RAM address 16.
                    sym_table.symbols.insert(value.to_string(), var_address.to_string());
                    sym_table.variable_count += 1;

                    var_address
                }
            }
        }
    };

    Instruction {
        line_type: Some(LineType::AInstruction),
        dest: None,
        comp: None,
        jump: None,
        value: Some(value)
    }
}


fn get_c_instruction(line: String) -> Instruction {
    if line.contains('=') && line.contains(';') {
        let parts: Vec<&str> = line.split('=').flat_map(|x| x.split(';')).collect();
        build_instruction(Some(LineType::CInstruction),
                          Some(parts[0].to_string()),
                          Some(parts[1].to_string()),
                          Some(parts[2].to_string()),
                          None)
    } else if line.contains('=') {
        let parts: Vec<&str> = line.split('=').collect();
        build_instruction(Some(LineType::CInstruction),
                          Some(parts[0].to_string()),
                          Some(parts[1].to_string()),
                          None, None)
    } else if line.contains(';') {
        let parts: Vec<&str> = line.split(';').collect();
        build_instruction(Some(LineType::CInstruction),
                          None,
                          Some(parts[0].to_string()),
                          Some(parts[1].to_string()),
                          None)
    } else {
        eprintln!("Error while parsing c instruction {}", line);
        process::exit(-1);
    }
}


pub fn parse_line(line: String, sym_table: &mut SymbolTable) -> Instruction {
    // We need to Separate 'line' depending on the type of instruction it is.
    if !is_instruction(&line) {
        return Instruction {
            line_type: None,
            dest: None,
            comp: None,
            jump: None,
            value: None
        }
    } 

    // Remove inline comments
    let line = if let Some(index) = line.find("//") {
        line[..index].trim()
    } else {
        line[..].trim()
    };

    if line.contains('@') {
        return get_a_instruction(line.to_string(), sym_table);
    }
    
    get_c_instruction(line.to_string())
}

pub fn is_label(line: &str) -> bool {
    line.starts_with('(') 
}

pub fn is_instruction(line: &str) -> bool {
    // If the line isn't a label, a comment or whitespace, it must be a command
    !line.trim().starts_with('(') && !line.trim().starts_with("//") && !line.is_empty()
}