enum SymbolKind {
    STATIC, 
    FIELD,
    ARG,
    VAR,
    NONE
}

enum SymbolType {
    INT,
    BOOL,
    CHAR,
    VOID
}

struct Symbol {
    name: String,
    symType: SymbolType,
    index: u32,
    kind: SymbolKind
}

pub struct SymbolTable {
    class_symbols: Vec<Symbol>,
    subroutine_symbols: Vec<Symbol>,
    num_static: u32,
    num_field: u32,
    num_arg: u32,
    num_var: u32
}


impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            class_symbols: Vec::new(),
            subroutine_symbols: Vec::new(),
            num_static: 0,
            num_arg: 0,
            num_field: 0,
            num_var: 0
        }
    }


    pub fn start_subroutine(&mut self) {
        // Resets num_arg and num_var and clears subroutine_symbols...
        unimplemented!()
    }


    pub fn define(&mut self, name: &str, symType: SymbolType, kind: SymbolKind) {
        // Adds a new symbol to the symbol table
        unimplemented!()
    }


    pub fn sym_count(&mut self, kind: SymbolKind) -> u32 {
        unimplemented!()
    }


    pub fn kind_of(&mut self, name: &String) -> &SymbolKind {
        unimplemented!()
    }


    pub fn type_of(&mut self, name: &String) -> &SymbolType {
        unimplemented!()
    }


    pub fn index_of(&mut self, name: &String) -> u32 {
        unimplemented!()
    }
}