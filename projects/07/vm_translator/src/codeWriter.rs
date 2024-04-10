/* ==================================================
 * Generates assembly code from the parsed VM command.
 * ================================================== */

use std::io::BufWriter;
use std::fs::File;

use crate::parser::{Command, CommandType};

pub struct CodeWriter {
    writer: BufWriter<File>
}

impl CodeWriter {
    pub fn new(output_file: String) -> CodeWriter {
        let file: File = File::open(output_file).expect("Couldn't open output file");

        CodeWriter {
            writer: BufWriter::new(file)
        }

    }
}

fn init(outputFile: String) {
    // Opens the output file for writing

}


fn writeArithmetic(comand: String) {
    // Translates an arithmetic command to the output
}

fn writePushPop(push_pop: String, segment: String, index: i32) {
    // Translates a push or pop command to the output

}
