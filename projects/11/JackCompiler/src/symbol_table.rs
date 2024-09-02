use std::fmt;

pub enum SymbolKind {       // This is according to the nand2tetris book definition
    STATIC, 
    FIELD,
    ARG,
    VAR
}

struct Symbol {
    name: String,
    symType: String,
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

/*
    We're assuming that the jack code is error free. Therefore we don't need to 
    keep track of class names or subroutine names...
 */
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
        self.subroutine_symbols.clear();
        self.num_arg = 0;
        self.num_var = 0;
    }


    pub fn define(&mut self, name: &str, symType: &String, kind: SymbolKind) {
        // Adds a new symbol to the appropriate symbol table
        let (symbols, counter) = match kind {
            SymbolKind::ARG    => (&mut self.subroutine_symbols, &mut self.num_arg),
            SymbolKind::VAR    => (&mut self.subroutine_symbols, &mut self.num_var),
            SymbolKind::FIELD  => (&mut self.class_symbols, &mut self.num_field),
            SymbolKind::STATIC => (&mut self.class_symbols, &mut self.num_static),
        };

        symbols.push(Symbol {
            name: name.to_string(),
            symType: symType.clone(),
            index: *counter,
            kind
        });
        *counter += 1;
    }


    pub fn sym_count(&mut self, kind: SymbolKind) -> u32 {
        match kind {
            SymbolKind::ARG    => self.num_arg,
            SymbolKind::VAR    => self.num_var,
            SymbolKind::FIELD  => self.num_field,
            SymbolKind::STATIC => self.num_static
        }
    }


    pub fn kind_of(&self, name: &String) -> Option<&SymbolKind> {
        for symbol in self.class_symbols.iter().chain(self.subroutine_symbols.iter()) {
            if name == &symbol.name {
                return Some(&symbol.kind)
            }
        }

        println!("NO SYMBOL FOUND (kind_of)");
        None
    }


    pub fn type_of(&self, name: &String) -> Option<&String> {
        for symbol in self.class_symbols.iter().chain(self.subroutine_symbols.iter()) {
            if name == &symbol.name {
                return Some(&symbol.symType)
            }
        }

        println!("NO SYMBOL FOUND (type_of)");
        None
    }


    pub fn index_of(&mut self, name: &String) -> Option<u32> {
        for symbol in self.class_symbols.iter().chain(self.subroutine_symbols.iter()) {
            if name == &symbol.name {
                return Some(symbol.index)
            }
        }

        println!("NO SYMBOL FOUND (index_of)");
        None
    }
}


impl fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::STATIC => write!(f, "STATIC"),
            Self::FIELD => write!(f, "FIELD"),
            Self::ARG => write!(f, "ARG"),
            Self::VAR => write!(f, "VAR")
        }
    }
} 