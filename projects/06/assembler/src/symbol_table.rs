use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub struct SymbolTable {
    pub symbols: HashMap<String, String>,
    pub variable_count: u32,
}

pub fn first_pass(reader: &BufReader<File>) -> SymbolTable {
    /*
     *  Does the first pass and builds up the symbol table.
     *  Incrememnt current command whenever a C or A instruction is encountered.
     *  It is not incremented when a label, pseudocommand or a comment is encountered 
     */

    // Initialise 
    // fn init....

}