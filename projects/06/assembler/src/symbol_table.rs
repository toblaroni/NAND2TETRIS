use std::collections::HashMap;

pub struct SymbolTable {
    pub symbols: HashMap<String, String>,
    pub variable_count: u32,
}

fn _find_address(symbol: &String, sym_table: &SymbolTable) -> i32 {
    0
}


fn _add_symbol(symbol: &String,
               address: &String,
               sym_table: &SymbolTable) {
}

pub fn init_sym_table() -> SymbolTable {
    /*
     *  Initialise the symbol table with the keywords.
     */

    let mut sym_table = SymbolTable {
        symbols: HashMap::new(),
        variable_count: 0
    };

    sym_table.symbols.insert(String::from("SP"),   String::from("0"));
    sym_table.symbols.insert(String::from("LCL"),  String::from("1"));
    sym_table.symbols.insert(String::from("ARG"),  String::from("2"));
    sym_table.symbols.insert(String::from("THIS"), String::from("3"));
    sym_table.symbols.insert(String::from("THAT"), String::from("4"));
    
    // R0 -> R15
    for i in 0..16 {
        let reg  = format!("R{}", i);
        let addr = i.to_string();
        sym_table.symbols.insert(reg, addr);
    }

    sym_table.symbols.insert(String::from("SCREEN"), String::from("16384"));
    sym_table.symbols.insert(String::from("KBD"),    String::from("24576"));

    sym_table
}