// Recursive top-down parser
use std::fs::File;
use std::io::{self, ErrorKind};
use std::path::PathBuf;

use crate::symbol_table::SymbolKind;
use crate::symbol_table::SymbolTable;
use crate::tokenizer::{TokenType, Tokenizer};
use crate::vm_writer::VMWriter;

pub enum VMSegment {
    Const,
    Arg,
    Local,
    Static,
    This,
    That,
    Pointer,
    Temp,
}

pub enum ArithmeticCommand {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Nor,
}

pub struct CompilationEngine {
    tokenizer: Tokenizer,
    vm_writer: VMWriter,
    symbol_table: SymbolTable,
    class_name: String,
}

impl CompilationEngine {
    pub fn new(source_file: PathBuf) -> Result<CompilationEngine, io::Error> {
        let mut output = source_file.clone();

        output.set_extension("vm");
        let output_file = File::create(&output)?;

        let vm_writer = VMWriter::new(output_file);
        let mut tokenizer = Tokenizer::new(&source_file)?;

        tokenizer.advance()?;

        let class_name = source_file
                        .file_stem()
                        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "No file name found in path."))?
                        .to_str()
                        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "File name is not valid UTF-8."))?
                        .to_owned();

        Ok(CompilationEngine {
            tokenizer,
            vm_writer,
            symbol_table: SymbolTable::new(),
            class_name
        })
    }

    pub fn compile_class(&mut self) -> Result<(), io::Error> {
        self.check_token(TokenType::Keyword, Some(&["class"]), false)?; // Class keyword

        self.check_token(TokenType::Identifier, None, false)?; // Class name

        self.check_token(TokenType::Symbol, Some(&["{"]), false)?;

        // 0 or more
        while self
            .check_token(TokenType::Keyword, Some(&["static", "field"]), true)
            .is_ok()
        {
            self.compile_class_var_dec()?;
        }

        // 0 or more
        while self
            .check_token(
                TokenType::Keyword,
                Some(&["constructor", "method", "function"]),
                true,
            )
            .is_ok()
        {
            self.compile_subroutine()?;
        }

        self.check_token(TokenType::Symbol, Some(&["}"]), false)?;

        self.vm_writer.close()
    }

    fn compile_class_var_dec(&mut self) -> Result<(), io::Error> {
        let sym_kind = match self.check_token(TokenType::Keyword, Some(&["static", "field"]), true)
        {
            Ok(()) => {
                self.tokenizer.advance()?;
                if "static" == self.tokenizer.get_current_token_value() {
                    SymbolKind::Static
                } else {
                    SymbolKind::Field
                }
            }
            Err(e) => return Err(e),
        };

        self.check_type(false)?;
        let sym_type = self.tokenizer.get_current_token_value();

        self.check_symbol(Some(sym_type.clone()), Some(sym_kind.clone()), true)?;

        while self
            .check_token(TokenType::Symbol, Some(&[","]), true)
            .is_ok()
        {
            self.tokenizer.advance()?;
            self.check_symbol(Some(sym_type.clone()), Some(sym_kind.clone()), true)?;
        }

        self.check_token(TokenType::Symbol, Some(&[";"]), false)?;

        Ok(())
    }

    fn compile_subroutine(&mut self) -> Result<(), io::Error> {
        self.check_token(
            TokenType::Keyword,
            Some(&["constructor", "function", "method"]),
            false,
        )?;

        // If constructor, insert code that allocates enough space for the class (aka)
        if self.tokenizer.get_current_token_value() == "constructor" {
            self.vm_writer.write_alloc(
                self.symbol_table.num_class_vars().to_string()
            )?;
        }

        // ('void' | type)
        match self.check_token(TokenType::Keyword, Some(&["void"]), true) {
            Ok(_) => {
                // Consume token
                self.tokenizer.advance()?;
            }
            Err(_) => {
                // Consumes and emits a type if there's one
                self.check_type(false)?;
            }
        }

        let ret_type = self.tokenizer.get_current_token_value();

        self.check_token(TokenType::Identifier, None, false)?;
        self.check_token(TokenType::Symbol, Some(&["("]), false)?;

        self.compile_param_list()?;
        self.check_token(TokenType::Symbol, Some(&[")"]), false)?;

        self.compile_subroutine_body()?;

        if ret_type == "void" {
            
        }

        Ok(())
    }

    fn compile_param_list(&mut self) -> Result<(), io::Error> {
        match self.check_type(true) {
            Ok(()) => {}
            Err(_) => {
                return Ok(());
            }
        };

        // Consume type
        self.tokenizer.advance()?;
        let mut sym_type = self.tokenizer.get_current_token_value();

        self.check_symbol(Some(sym_type), Some(SymbolKind::Arg), true)?;

        while self
            .check_token(TokenType::Symbol, Some(&[","]), true)
            .is_ok()
        {
            // consume ','
            self.tokenizer.advance()?;

            self.check_type(true)?;
            self.tokenizer.advance()?;
            sym_type = self.tokenizer.get_current_token_value();

            self.check_symbol(Some(sym_type), Some(SymbolKind::Arg), true)?;
        }

        Ok(())
    }

    fn compile_subroutine_body(&mut self) -> Result<(), io::Error> {
        self.check_token(TokenType::Symbol, Some(&["{"]), false)?;

        while self
            .check_token(TokenType::Keyword, Some(&["var"]), true)
            .is_ok()
        {
            self.compile_var_dec()?
        }

        self.compile_statements()?;


        self.check_token(TokenType::Symbol, Some(&["}"]), false)?;
        Ok(())
    }

    fn compile_var_dec(&mut self) -> Result<(), io::Error> {
        self.check_token(TokenType::Keyword, Some(&["var"]), false)?;
        self.check_type(false)?;
        let sym_type = self.tokenizer.get_current_token_value();

        self.check_symbol(Some(sym_type.clone()), Some(SymbolKind::Var), true)?;

        while self
            .check_token(TokenType::Symbol, Some(&[","]), true)
            .is_ok()
        {
            self.tokenizer.advance()?;

            self.check_symbol(Some(sym_type.clone()), Some(SymbolKind::Var), true)?;
        }

        self.check_token(TokenType::Symbol, Some(&[";"]), false)?;

        Ok(())
    }

    fn compile_statements(&mut self) -> Result<(), io::Error> {
        loop {
            let token = if let Some(t) = self.tokenizer.peek() {
                t
            } else {
                return Err(self.compilation_error(
                    "Expected ['let', 'if', 'while', 'do', 'return'. No token was found.",
                ));
            };

            match token.get_value().as_str() {
                "let" => self.compile_let()?,
                "if" => self.compile_if()?,
                "while" => self.compile_while()?,
                "do" => self.compile_do()?,
                "return" => self.compile_return()?,
                _ => break,
            }
        }

        Ok(())
    }

    fn compile_do(&mut self) -> Result<(), io::Error> {
        // Can just advance and emit since we know 'do' must be next
        self.tokenizer.advance()?;

        self.check_symbol(None, None, false)?;

        if let Ok(_) = self.check_token(TokenType::Symbol, Some(&["."]), true) {
            // .subroutineName
            self.tokenizer.advance()?;
            self.check_symbol(None, None, false)?;
        }

        self.check_token(TokenType::Symbol, Some(&["("]), false)?;
        self.compile_expression_list()?;
        self.check_token(TokenType::Symbol, Some(&[")"]), false)?;
        self.check_token(TokenType::Symbol, Some(&[";"]), false)?;

        Ok(())
    }

    fn compile_let(&mut self) -> Result<(), io::Error> {
        self.check_token(TokenType::Keyword, Some(&["let"]), false)?;
        self.check_symbol(None, None, false)?;

        if let Ok(_) = self.check_token(TokenType::Symbol, Some(&["["]), true) {
            self.tokenizer.advance()?;

            self.compile_expression()?;

            self.check_token(TokenType::Symbol, Some(&["]"]), false)?;
        }

        self.check_token(TokenType::Symbol, Some(&["="]), false)?;
        self.compile_expression()?;
        self.check_token(TokenType::Symbol, Some(&[";"]), false)?;

        Ok(())
    }

    fn compile_while(&mut self) -> Result<(), io::Error> {
        self.check_token(TokenType::Keyword, Some(&["while"]), false)?;

        self.check_token(TokenType::Symbol, Some(&["("]), false)?;
        self.compile_expression()?;
        self.check_token(TokenType::Symbol, Some(&[")"]), false)?;

        self.check_token(TokenType::Symbol, Some(&["{"]), false)?;
        self.compile_statements()?;
        self.check_token(TokenType::Symbol, Some(&["}"]), false)?;

        Ok(())
    }

    fn compile_return(&mut self) -> Result<(), io::Error> {
        self.check_token(TokenType::Keyword, Some(&["return"]), false)?;

        if self
            .check_token(TokenType::Symbol, Some(&[";"]), true)
            .is_err()
        {
            self.compile_expression()?;
        }

        self.check_token(TokenType::Symbol, Some(&[";"]), false)?;
        Ok(())
    }

    fn compile_if(&mut self) -> Result<(), io::Error> {
        self.check_token(TokenType::Keyword, Some(&["if"]), false)?;

        self.check_token(TokenType::Symbol, Some(&["("]), false)?;
        self.compile_expression()?;
        self.check_token(TokenType::Symbol, Some(&[")"]), false)?;

        self.check_token(TokenType::Symbol, Some(&["{"]), false)?;
        self.compile_statements()?;
        self.check_token(TokenType::Symbol, Some(&["}"]), false)?;

        if let Ok(_) = self.check_token(TokenType::Keyword, Some(&["else"]), true) {
            self.tokenizer.advance()?;
            self.check_token(TokenType::Symbol, Some(&["{"]), false)?;
            self.compile_statements()?;
            self.check_token(TokenType::Symbol, Some(&["}"]), false)?;
        }

        Ok(())
    }

    fn compile_expression_list(&mut self) -> Result<(), io::Error> {
        if self
            .check_token(TokenType::Symbol, Some(&[")"]), true)
            .is_ok()
        {
            return Ok(());
        }

        self.compile_expression()?;

        while self
            .check_token(TokenType::Symbol, Some(&[","]), true)
            .is_ok()
        {
            self.tokenizer.advance()?;
            self.compile_expression()?;
        }

        Ok(())
    }

    fn compile_expression(&mut self) -> Result<(), io::Error> {
        self.compile_term()?;

        let ops = &["+", "-", "*", "/", "&", "|", "<", ">", "="];

        // (op term)*
        while self.check_token(TokenType::Symbol, Some(ops), true).is_ok() {
            self.tokenizer.advance()?;
            self.compile_term()?;
        }

        Ok(())
    }

    fn compile_term(&mut self) -> Result<(), io::Error> {
        if let Some(t) = self.tokenizer.peek() {
            match t.get_token_type() {
                TokenType::IntConst => self.check_token(TokenType::IntConst, None, false)?,
                TokenType::StringConst => self.check_token(TokenType::StringConst, None, false)?,
                TokenType::Keyword => self.check_token(
                    TokenType::Keyword,
                    Some(&["true", "false", "null", "this"]),
                    false,
                )?,
                TokenType::Identifier => self.handle_term_id()?,
                TokenType::Symbol => self.handle_term_symbol()?,
            }
        }

        Ok(())
    }

    fn handle_term_id(&mut self) -> Result<(), io::Error> {
        // varname | varname [expression] | subroutineCall
        // Consume the id
        self.check_symbol(None, None, false)?;

        if let Some(t) = self.tokenizer.peek() {
            match t.get_value().as_str() {
                "[" => {
                    self.tokenizer.advance()?;
                    self.compile_expression()?;
                    self.check_token(TokenType::Symbol, Some(&["]"]), false)?;
                }
                "(" => {
                    // Subroutine call
                    self.tokenizer.advance()?;
                    self.compile_expression_list()?;
                    self.check_token(TokenType::Symbol, Some(&[")"]), false)?;
                }
                "." => {
                    // Subroutine call
                    // .subroutine_name(expressionlist)
                    self.tokenizer.advance()?;
                    self.check_symbol(None, None, false)?;
                    self.check_token(TokenType::Symbol, Some(&["("]), false)?;
                    self.compile_expression_list()?;
                    self.check_token(TokenType::Symbol, Some(&[")"]), false)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_term_symbol(&mut self) -> Result<(), io::Error> {
        if let Some(t) = self.tokenizer.peek() {
            match t.get_value().as_str() {
                "(" => {
                    self.tokenizer.advance()?;
                    self.compile_expression()?;
                    self.check_token(TokenType::Symbol, Some(&[")"]), false)?;
                }
                "-" | "~" => {
                    // Unary-op
                    self.tokenizer.advance()?;
                    self.compile_term()?;
                }
                _ => {
                    return Err(self.compilation_error(&format!(
                        "Invalid symbol encountered while parsing term: {}",
                        t.get_value()
                    )))
                }
            }
        }

        Ok(())
    }

    fn check_token(
        &mut self,
        token_type: TokenType,
        values: Option<&[&str]>,
        peek: bool,
    ) -> Result<(), io::Error> {
        /*
           Checks current token or next token with the token_type and values (if any).
           When checking current value it advances the tokenizer and emits the token in xml.
        */

        let ct = if peek {
            self.tokenizer
                .peek()
                .ok_or_else(|| self.compilation_error("There is no next token."))?
        } else {
            self.tokenizer.advance()?;
            self.tokenizer
                .current_token()
                .ok_or_else(|| self.compilation_error("There is no current token."))?
        };

        let token_value = ct.get_value();
        let ctoken_type = ct.get_token_type();

        let value_check = values.map_or(true, |vals| vals.contains(&token_value.as_str()));

        if token_type != *ctoken_type || !value_check {
            return Err(
                self.compilation_error(
                    &format!(
                        "Expected a {:?} with one of the following values: {:?}. Found a {} with value '{}' instead",
                        token_type,
                        values.unwrap_or(&["Any valid value"]),
                        token_type,
                        token_value
                    )
                )
            );
        }

        Ok(())
    }

    fn check_symbol(
        &mut self,
        sym_type: Option<String>,
        sym_kind: Option<SymbolKind>,
        being_defined: bool,
    ) -> Result<(), io::Error> {
        /*
            This will handle identifiers and emitting them.
            Check for ID, add to symbol table and emit to xml for now...

            If being_defined then we want to add the symbol table and use the values passed to the function.
            else we want to find the symbol in the symbol table and use those values.
        */

        self.tokenizer.advance()?;
        let ct = self
            .tokenizer
            .current_token()
            .ok_or_else(|| self.compilation_error("There is no current token."))?;

        if *ct.get_token_type() != TokenType::Identifier {
            return Err(self.compilation_error("Expected an identifier."));
        }

        let sym_name = self.tokenizer.get_current_token_value();

        let (sym_type, sym_kind) = if being_defined {
            let sym_type =
                sym_type.ok_or_else(|| self.compilation_error("Symbol type is missing"))?;
            let sym_kind =
                sym_kind.ok_or_else(|| self.compilation_error("Symbol kind is missing"))?;
            self.symbol_table
                .define(&sym_name, &sym_type, sym_kind.clone());
            (sym_type, sym_kind)
        } else {
            // We need to get the kind and type from the symbol table
            let sym_type = self.symbol_table.type_of(&sym_name).to_owned();
            let sym_kind = self.symbol_table.kind_of(&sym_name).clone();
            (sym_type, sym_kind)
        };

        Ok(()) // Maybe should return sym_type and sym_return... we shall see
    }

    fn check_type(&mut self, next_token: bool) -> Result<(), io::Error> {
        let ct = if next_token {
            self.tokenizer
                .peek()
                .ok_or_else(|| self.compilation_error("There is no next token."))?
        } else {
            self.tokenizer.advance()?;
            self.tokenizer
                .current_token()
                .ok_or_else(|| self.compilation_error("There is no current token."))?
        };

        let token_value = ct.get_value();
        let token_type = ct.get_token_type();

        if *token_type == TokenType::Identifier {
            return Ok(());
        }

        if *token_type == TokenType::Keyword
            && ["int", "char", "boolean"].contains(&token_value.as_str())
        {
            return Ok(());
        }

        Err(self.compilation_error("Expected either [int | char | boolean | className]."))
    }

    // Would be better if we had our own custom error type
    fn compilation_error(&self, error: &str) -> io::Error {
        // This is perhaps not idiomatic for rust?...
        io::Error::new(
            ErrorKind::InvalidInput,
            format!(
                "{}. Line {} in file {}.",
                error,
                self.tokenizer.get_line_number(),
                self.tokenizer.get_file_name()
            ),
        )
    }
}
