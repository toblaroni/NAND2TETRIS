use std::{fmt, process::exit};

pub enum SymbolKind {
    // This is according to the nand2tetris book definition
    Static,
    Field,
    Arg,
    Var,
    None, // Subroutines and Classes
}

struct Symbol {
    name: String,
    symType: String,
    index: u32,
    kind: SymbolKind,
}

pub struct SymbolTable {
    class_symbols: Vec<Symbol>,
    subroutine_symbols: Vec<Symbol>,
    num_static: u32,
    num_field: u32,
    num_arg: u32,
    num_var: u32,
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
            num_var: 0,
        }
    }

    pub fn start_subroutine(&mut self) {
        // Resets num_arg and num_var and clears subroutine_symbols...
        self.subroutine_symbols.clear();
        self.num_arg = 0;
        self.num_var = 0;
    }

    pub fn define(&mut self, name: &str, symType: &str, kind: SymbolKind) {
        // Adds a new symbol to the appropriate symbol table
        let (symbols, counter) = match kind {
            SymbolKind::Arg => (&mut self.subroutine_symbols, &mut self.num_arg),
            SymbolKind::Var => (&mut self.subroutine_symbols, &mut self.num_var),
            SymbolKind::Field => (&mut self.class_symbols, &mut self.num_field),
            SymbolKind::Static => (&mut self.class_symbols, &mut self.num_static),
            _ => return,
        };

        symbols.push(Symbol {
            name: name.to_string(),
            symType: symType.to_string(),
            index: *counter,
            kind,
        });
        *counter += 1;
    }

    pub fn sym_count(&mut self, kind: SymbolKind) -> u32 {
        match kind {
            SymbolKind::Arg => self.num_arg,
            SymbolKind::Var => self.num_var,
            SymbolKind::Field => self.num_field,
            SymbolKind::Static => self.num_static,
            _ => 0,
        }
    }

    pub fn num_class_vars(&self) -> usize {
        self.class_symbols.len()
    }

    pub fn kind_of(&self, name: &String) -> &SymbolKind {
        for symbol in self
            .class_symbols
            .iter()
            .chain(self.subroutine_symbols.iter())
        {
            if name == &symbol.name {
                return &symbol.kind;
            }
        }

        &SymbolKind::None
    }

    pub fn type_of(&self, name: &String) -> &str {
        for symbol in self
            .class_symbols
            .iter()
            .chain(self.subroutine_symbols.iter())
        {
            if name == &symbol.name {
                return &symbol.symType;
            }
        }

        return "class/subroutine";
    }

    pub fn index_of(&mut self, name: &String) -> Option<u32> {
        for symbol in self
            .class_symbols
            .iter()
            .chain(self.subroutine_symbols.iter())
        {
            if name == &symbol.name {
                return Some(symbol.index);
            }
        }

        None
    }
}

impl fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Static => write!(f, "Static"),
            Self::Field => write!(f, "Field"),
            Self::Arg => write!(f, "Arg"),
            Self::Var => write!(f, "Var"),
            Self::None => write!(f, "None"),
        }
    }
}

impl Clone for SymbolKind {
    fn clone(&self) -> SymbolKind {
        match self {
            Self::Arg => SymbolKind::Arg,
            Self::Static => SymbolKind::Static,
            Self::Field => SymbolKind::Field,
            Self::Var => SymbolKind::Var,
            Self::None => SymbolKind::None,
        }
    }
}
