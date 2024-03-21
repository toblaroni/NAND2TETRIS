use crate::parser::{Instruction, LineType};
fn translate_a_instruction(ins: Instruction) -> String {
    match ins.value {
        Some(uint) => format!("0{:015b}", uint),
        None       => panic!("Error while translating A instruction")
    }
}



fn translate_c_instruction(ins: Instruction) -> String {
    let dest_code = match ins.dest.as_deref() {
        None        => "000",
        Some("M")   => "001",
        Some("D")   => "010",
        Some("MD")  => "011",
        Some("A")   => "100",
        Some("AM")  => "101",
        Some("AD")  => "110",
        Some("AMD") => "111",
        _ => panic!("Invalid dest mnemonic")
    };

    let comp_code = match ins.comp.as_deref() {
        Some("0")   => "0101010", // a = 0
        Some("1")   => "0111111",
        Some("-1")  => "0111010",
        Some("D")   => "0001100",
        Some("A")   => "0110000",
        Some("!D")  => "0001101",
        Some("!A")  => "0110001",
        Some("-D")  => "0001111",
        Some("-A")  => "0110011",
        Some("D+1") => "0011111",
        Some("A+1") => "0110111",
        Some("D-1") => "0001110",
        Some("A-1") => "0110010",
        Some("D+A") => "0000010",
        Some("D-A") => "0010011",
        Some("A-D") => "0000111",
        Some("D&A") => "0000000",
        Some("D|A") => "0010101",
        Some("M")   => "1110000", // a = 1
        Some("!M")  => "1110001",
        Some("-M")  => "1110011",
        Some("M+1") => "1110111",
        Some("M-1") => "1110010",
        Some("D+M") => "1000010",
        Some("D-M") => "1010011",
        Some("M-D") => "1000111",
        Some("D&M") => "1000000",
        Some("D|M") => "1010101",
        _           => panic!("Invalid comp mnemonic")
    };

    let jump_code = match ins.jump.as_deref() {
        None        => "000",
        Some("JGT") => "001",
        Some("JEQ") => "010",
        Some("JGE") => "011",
        Some("JLT") => "100",
        Some("JNE") => "101",
        Some("JLE") => "110",
        Some("JMP") => "111",
        _ => panic!("Invalid jump mnemonic")
    };

    String::from("111") + dest_code + comp_code + jump_code
}

pub fn translate_instruction(ins: Instruction) -> String {
    match ins.line_type {
        Some(LineType::AInstruction) => translate_a_instruction(ins),
        Some(LineType::CInstruction) => translate_c_instruction(ins),
        None => panic!("Instruction is neither type A or C.")
    }
}