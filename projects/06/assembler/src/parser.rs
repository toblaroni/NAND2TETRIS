use core::fmt;
use std::process;

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

fn get_a_instruction(line: String) -> Instruction {

    let value: u32 = line[1..]
                    .trim()
                    .parse()
                    .expect("Bad a instruction value");

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
        eprintln!("Error while parsing instruction {}", line);
        process::exit(-1);
    }
}


pub fn parse_line(line: String) -> Instruction {
    // We need to Separate 'line' depending on the type of instruction it is.
    if line.starts_with("//") || line.is_empty() {
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
        return get_a_instruction(line.to_string());
    }
    
    get_c_instruction(line.to_string())
}

pub fn is_label(line: &String) -> bool {
    line.starts_with("(") 
}

pub fn is_instruction(line: &String) -> bool {
    // If the line isn't a label, a comment or whitespace, it must be a command
    !line.trim().starts_with("(") && !line.trim().starts_with("//") && !line.is_empty()
}