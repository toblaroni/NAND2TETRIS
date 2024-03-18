enum LineType {
    CInstruction,
    AInstruction,
    Comment
}

struct Instruction {
    line_type: LineType,

    dest: Option<String>,  
    comp: Option<String>,
    jump: Option<String>, 

    value: Option<u32>
}


fn get_a_instruction(line: String) -> Instruction {
    let value: u32 = &line[1..];

    return Instruction {
        line_type: LineType::AInstruction,
            dest: None,
            comp: None,
            jump: None,
            value: value
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
    
    // Loop through the line
    
}

