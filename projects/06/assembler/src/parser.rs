use std::process;

enum LineType {
    CInstruction,
    AInstruction,
    Comment
}

pub struct Instruction {
    pub line_type: LineType,

    // C-Instruction
    pub dest: Option<String>,  
    pub comp: Option<String>,
    pub jump: Option<String>, 

    // a-Instruction
    pub value: Option<u32>
}


fn get_a_instruction(line: String) -> Instruction {
    let value: u32 = line[1..].parse().expect("Bad a instruction value");
    println!("Value {}", &value);

    return Instruction {
        line_type: LineType::AInstruction,
            dest: None,
            comp: None,
            jump: None,
            value: Some(value)
    }
}


fn get_c_instruction(line: String) -> Instruction {
    let parts: Vec<&str> = line.split('=').flat_map(|x| x.split(';')).collect();

    match parts.len() {
        3 => {
        }
    }
}


pub fn parse(line: String) -> Instruction {
    // We need to Separate 'line' depending on the type of instruction it is.
    if line.starts_with("//") {
        return Instruction {
            line_type: LineType::Comment,
            dest: None,
            comp: None,
            jump: None,
            value: None
        }
    }

    if line.contains('@') {
        return get_a_instruction(line);
    }
    
    get_c_instruction(line)
}

