
use std::process;

use crate::parser::{Instruction, LineType};




fn translate_a_instruction(ins: Instruction) -> String {
    match ins.value {
        Some(uint) => {
            format!("{:015b}", uint)
        }
        None => {
            println!("Something has gone very wrong");
            process::exit(-1);
        }
    }
}

fn translate_c_instruction(ins: Instruction) -> String {
    String::new()
}

pub fn translate_instruction(ins: Instruction) -> String {
    match ins.line_type {
        Some(LineType::AInstruction) => translate_a_instruction(ins),
        Some(LineType::CInstruction) => translate_c_instruction(ins),
        None => {
            println!("Instruction is neither type A or C.");
            process::exit(-1)
        }
    }
}