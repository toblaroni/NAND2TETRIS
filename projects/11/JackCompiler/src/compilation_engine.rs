// Recursive top-down parser
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use std::fs::File;

use crate::symbol_table::SymbolTable;
use crate::tokenizer::{Tokenizer, TokenType};
use crate::vm_writer::VMWriter;
use crate::symbol_table::SymbolKind;

pub enum VMSegment { CONST, ARG, LOCAL, STATIC, THIS, THAT, POINTER, TEMP }
pub enum ArithmeticCommand { ADD, SUB, NEG, EQ, GT, LT, AND, OR, NOT }

pub struct CompilationEngine {
    tokenizer: Tokenizer,
    vm_writer: VMWriter,
    symbol_table: SymbolTable
}

impl CompilationEngine {
    pub fn new(source_file: PathBuf) -> Result<CompilationEngine, io::Error> {
        let mut output = source_file.clone();

        output.set_extension("xml");
        let output_file = File::create(&output)?;
        
        let vm_writer = VMWriter::new(output_file);
        let mut tokenizer = Tokenizer::new(source_file)?;

        tokenizer.advance()?;

        Ok(CompilationEngine {
            tokenizer,
            vm_writer,
            symbol_table: SymbolTable::new()
        })
    }

    pub fn compile_class(&mut self) -> Result<(), io::Error> {
        self.vm_writer.write_all("<class>\n".as_bytes())?;

        self.check_token(TokenType::Keyword, Some(&["class"]), false)?;     // Class keyword

        self.check_token(TokenType::Identifier, None, false)?;               // Class name

        self.check_token(TokenType::Symbol, Some(&["{"]), false)?;

        // 0 or more
        while self.check_token(TokenType::Keyword, Some(&["static", "field"]), true).is_ok() {
            self.compile_class_var_dec()?;
        }

        // 0 or more
        while self.check_token(TokenType::Keyword, Some(&["constructor", "method", "function"]), true).is_ok() {
            self.compile_subroutine()?;
        }

        self.check_token(TokenType::Symbol, Some(&["}"]), false)?;
       
        self.vm_writer.write_all("</class>\n".as_bytes())?;
        Ok(())
    }


    fn compile_class_var_dec(&mut self) -> Result<(), io::Error> {
        self.vm_writer.write_all("<classVarDec>\n".as_bytes())?;


        let sym_kind = match self.check_token(TokenType::Keyword, Some(&["static", "field"]), true) {
            Ok(()) => {
                self.tokenizer.advance()?;
                self.emit_token()?;
                if "static" == self.tokenizer.get_current_token_value() {
                    SymbolKind::STATIC
                } else {
                    SymbolKind::FIELD
                }
            }
            Err(e) => return Err(e)
        };

        self.check_type(false)?;
        let sym_type = self.tokenizer.get_current_token_value();

        match self.check_token(TokenType::Identifier, None, true) {
            Ok(()) => {
                self.tokenizer.advance()?;
                let sym_name = self.tokenizer.get_current_token_value();        
                self.emit_symbol(
                    sym_name,
                    sym_type,
                    sym_kind,
                    true
                )?;
            }
            Err(e) => return Err(e)
        };


        while self.check_token(TokenType::Symbol, Some(&[","]), true).is_ok() {
            self.tokenizer.advance()?;
            self.emit_token()?;         // Emit the , symbol
            self.check_token(TokenType::Identifier, None, false)?;   // Consume and emit identifier
        }

        self.check_token(TokenType::Symbol, Some(&[";"]), false)?;

        self.vm_writer.write_all("</classVarDec>\n".as_bytes())?;
        Ok(())
    }


    fn compile_subroutine(&mut self) -> Result<(), io::Error> {
        self.vm_writer.write_all("<subroutineDec>\n".as_bytes())?;
        self.check_token(TokenType::Keyword, Some(&["constructor", "function", "method"]), false)?;

        // ('void' | type)
        match self.check_token(TokenType::Keyword, Some(&["void"]), true) {
            Ok(_) => {
                // Consume token and emit
                self.tokenizer.advance()?;
                self.emit_token()?;
            }
            Err(_) => {
                // Consumes and emits a type if there's one
                self.check_type(false)?;
            }
        }

        self.check_token(TokenType::Identifier, None, false)?;
        self.check_token(TokenType::Symbol, Some(&["("]), false)?;

        self.compile_param_list()?;
        self.check_token(TokenType::Symbol, Some(&[")"]), false)?;

        self.compile_subroutine_body()?;

        self.vm_writer.write_all("</subroutineDec>\n".as_bytes())?;
        Ok(())
    }

    fn compile_param_list(&mut self) -> Result<(), io::Error> {
        self.vm_writer.write_all("<parameterList>\n".as_bytes())?;

        match self.check_type(true) {
            Ok(()) => {},
            Err(_) => {
                self.vm_writer.write_all("</parameterList>\n".as_bytes())?;
                return Ok(())
            }
        };

        // Consume type
        self.tokenizer.advance()?;
        self.emit_token()?;

        self.check_token(TokenType::Identifier, None, false)?;

        while self.check_token(TokenType::Symbol, Some(&[","]), true).is_ok() {
            // consume ','
            self.tokenizer.advance()?;
            self.emit_token()?;

            self.check_type(false)?;
            self.check_token(TokenType::Identifier, None, false)?;
        }

        self.vm_writer.write_all("</parameterList>\n".as_bytes())?;
        Ok(())
    }

    fn compile_subroutine_body(&mut self) -> Result<(), io::Error> {
        self.vm_writer.write_all("<subroutineBody>\n".as_bytes())?;
        self.check_token(TokenType::Symbol, Some(&["{"]), false)?;

        while self.check_token(TokenType::Keyword, Some(&["var"]), true).is_ok() {
            self.compile_var_dec()?
        }

        self.compile_statements()?;

        self.check_token(TokenType::Symbol, Some(&["}"]), false)?;
        self.vm_writer.write_all("</subroutineBody>\n".as_bytes())?;
        Ok(())
    }

    fn compile_var_dec(&mut self) -> Result<(), io::Error> {
        self.vm_writer.write_all("<varDec>\n".as_bytes())?;

        self.check_token(TokenType::Keyword, Some(&["var"]), false)?;
        self.check_type(false)?;
        self.check_token(TokenType::Identifier, None, false)?;

        while self.check_token(TokenType::Symbol, Some(&[","]), true).is_ok() {
            self.tokenizer.advance()?;
            self.emit_token()?;

            self.check_token(TokenType::Identifier, None, false)?;
        }

        self.check_token(TokenType::Symbol, Some(&[";"]), false)?;

        self.vm_writer.write_all("</varDec>\n".as_bytes())?;
        Ok(())
    }

    fn compile_statements(&mut self) -> Result<(), io::Error> {
        self.vm_writer.write_all("<statements>\n".as_bytes())?;

        loop {
            let token = if let Some(t) = self.tokenizer.peek() {
                t
            } else {
                return Err(self.compilation_error("Expected ['let', 'if', 'while', 'do', 'return'. No token was found."))
            };

            match token.get_value().as_str() {
                "let"    => self.compile_let()?,
                "if"     => self.compile_if()?,
                "while"  => self.compile_while()?,
                "do"     => self.compile_do()?,
                "return" => self.compile_return()?,
                _        => break
            }

        }

        self.vm_writer.write_all("</statements>\n".as_bytes())?;
        Ok(())
    }

    fn compile_do(&mut self) -> Result<(), io::Error> {
        self.vm_writer.write_all("<doStatement>\n".as_bytes())?;
        
        // Can just advance and emit since we know 'do' must be next
        self.tokenizer.advance()?; 
        self.emit_token()?;
        
        self.check_token(TokenType::Identifier, None, false)?;

        if let Ok(_) = self.check_token(TokenType::Symbol, Some(&["."]), true) {
            // .subroutineName
            self.tokenizer.advance()?;
            self.emit_token()?;
            self.check_token(TokenType::Identifier, None, false)?;
        }

        self.check_token(TokenType::Symbol, Some(&["("]), false)?;
        self.compile_expression_list()?;
        self.check_token(TokenType::Symbol, Some(&[")"]), false)?;
        self.check_token(TokenType::Symbol, Some(&[";"]), false)?;

        self.vm_writer.write_all("</doStatement>\n".as_bytes())?;
        Ok(())
    }

    fn compile_let(&mut self) -> Result<(), io::Error> {
        self.vm_writer.write_all("<letStatement>\n".as_bytes())?;

        self.check_token(TokenType::Keyword, Some(&["let"]), false)?;
        self.check_token(TokenType::Identifier, None, false)?;

        if let Ok(_) = self.check_token(TokenType::Symbol, Some(&["["]), true) {
            self.tokenizer.advance()?;
            self.emit_token()?;

            self.compile_expression()?;

            self.check_token(TokenType::Symbol, Some(&["]"]), false)?;
        }

        self.check_token(TokenType::Symbol, Some(&["="]), false)?;
        self.compile_expression()?;
        self.check_token(TokenType::Symbol, Some(&[";"]), false)?;

        self.vm_writer.write_all("</letStatement>\n".as_bytes())?;
        Ok(())
    }

    fn compile_while(&mut self) -> Result<(), io::Error> {
        self.vm_writer.write_all("<whileStatement>\n".as_bytes())?;
        self.check_token(TokenType::Keyword, Some(&["while"]), false)?;

        self.check_token(TokenType::Symbol, Some(&["("]), false)?;
        self.compile_expression()?;        
        self.check_token(TokenType::Symbol, Some(&[")"]), false)?;

        self.check_token(TokenType::Symbol, Some(&["{"]), false)?;
        self.compile_statements()?;
        self.check_token(TokenType::Symbol, Some(&["}"]), false)?;

        self.vm_writer.write_all("</whileStatement>\n".as_bytes())?;
        Ok(())
    }

    
    fn compile_return(&mut self) -> Result<(), io::Error> {
        self.vm_writer.write_all("<returnStatement>\n".as_bytes())?;

        self.check_token(TokenType::Keyword, Some(&["return"]), false)?;

        if self.check_token(TokenType::Symbol, Some(&[";"]), true).is_err() {
            self.compile_expression()?;
        }

        self.check_token(TokenType::Symbol, Some(&[";"]), false)?;
        self.vm_writer.write_all("</returnStatement>\n".as_bytes())?;
        Ok(())
    }


    fn compile_if(&mut self) -> Result<(), io::Error> {
        self.vm_writer.write_all("<ifStatement>\n".as_bytes())?;
        self.check_token(TokenType::Keyword, Some(&["if"]), false)?;

        self.check_token(TokenType::Symbol, Some(&["("]), false)?;
        self.compile_expression()?;
        self.check_token(TokenType::Symbol, Some(&[")"]), false)?;

        self.check_token(TokenType::Symbol, Some(&["{"]), false)?;
        self.compile_statements()?;
        self.check_token(TokenType::Symbol, Some(&["}"]), false)?;

        if let Ok(_) = self.check_token(TokenType::Keyword, Some(&["else"]), true) {
            self.tokenizer.advance()?;
            self.emit_token()?;
            self.check_token(TokenType::Symbol, Some(&["{"]), false)?;
            self.compile_statements()?;
            self.check_token(TokenType::Symbol, Some(&["}"]), false)?;
        }

        self.vm_writer.write_all("</ifStatement>\n".as_bytes())?;
        Ok(())
    }


    fn compile_expression_list(&mut self) -> Result<(), io::Error> {
        self.vm_writer.write_all("<expressionList>\n".as_bytes())?;

        if let Ok(_) = self.check_token(TokenType::Symbol, Some(&[")"]), true) {
            self.vm_writer.write_all("</expressionList>\n".as_bytes())?;
            return Ok(())
        }

        self.compile_expression()?;

        while let Ok(_) = self.check_token(TokenType::Symbol, Some(&[","]), true) {
            self.tokenizer.advance()?;
            self.emit_token()?;
            self.compile_expression()?;
        }

        self.vm_writer.write_all("</expressionList>\n".as_bytes())?;
        Ok(())
    }


    fn compile_expression(&mut self) -> Result<(), io::Error> {
        self.vm_writer.write_all("<expression>\n".as_bytes())?;
        self.compile_term()?;

        let ops = &["+", "-", "*", "/", "&", "|", "<", ">", "="];

        // (op term)*
        while self.check_token(TokenType::Symbol, Some(ops), true).is_ok() {
            self.tokenizer.advance()?;
            self.emit_token()?;
            self.compile_term()?;
        }

        self.vm_writer.write_all("</expression>\n".as_bytes())?;
        Ok(())
    }

    fn compile_term(&mut self) -> Result<(), io::Error> {
        self.vm_writer.write_all("<term>\n".as_bytes())?;

        if let Some(t) = self.tokenizer.peek() {
            match t.get_token_type() {
                TokenType::IntConst    => self.check_token(TokenType::IntConst, None, false)?,
                TokenType::StringConst => self.check_token(TokenType::StringConst, None, false)?,
                TokenType::Keyword     => self.check_token(TokenType::Keyword, Some(&["true", "false", "null", "this"]), false)?,
                TokenType::Identifier  => self.handle_term_id()?,
                TokenType::Symbol      => self.handle_term_symbol()?
            }
        }

        self.vm_writer.write_all("</term>\n".as_bytes())?;
        Ok(())
    }

    fn handle_term_id(&mut self) -> Result<(), io::Error> {
        // varname | varname [expression] | subroutineCall 
        // Consume the id
        self.tokenizer.advance()?;
        self.emit_token()?;

        if let Some(t) = self.tokenizer.peek() {
            match t.get_value().as_str() {
                "[" => {
                    self.tokenizer.advance()?;
                    self.emit_token()?;
                    self.compile_expression()?;
                    self.check_token(TokenType::Symbol, Some(&["]"]), false)?;
                }
                "(" => {
                    // Subroutine call
                    self.tokenizer.advance()?;
                    self.emit_token()?;
                    self.compile_expression_list()?;
                    self.check_token(TokenType::Symbol, Some(&[")"]), false)?;
                }
                "." => {
                    // Subroutine call
                    // .subroutine_name(expressionlist)
                    self.tokenizer.advance()?;
                    self.emit_token()?;
                    self.check_token(TokenType::Identifier, None, false)?;
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
                    self.emit_token()?;
                    self.compile_expression()?;
                    self.check_token(TokenType::Symbol, Some(&[")"]), false)?;

                }
                "-" | "~" => {  // Unary-op
                    self.tokenizer.advance()?;
                    self.emit_token()?;
                    self.compile_term()?;
                }
                _ => return Err(
                    self.compilation_error(
                        &format!("Invalid symbol encountered while parsing term: {}", t.get_value())
                    )
                )
            }
        }

        Ok(())
    }


    fn emit_symbol(
        &mut self,
        sym_name: String,
        sym_type: String,
        sym_kind: SymbolKind,
        is_defined: bool
    ) -> Result<(), io::Error> {
        self.vm_writer.write_all("<identifier>\n".as_bytes())?;
        self.vm_writer.write_all(format!("<name>{}</name>", sym_name).as_bytes())?;
        self.vm_writer.write_all(format!("<type>{}</type>", sym_type).as_bytes())?;
        self.vm_writer.write_all(format!("<kind>{}</kind>", sym_kind).as_bytes())?;
        self.vm_writer.write_all(format!("<defined>{}</defined>", is_defined).as_bytes())?;
        self.vm_writer.write_all("</identifier>\n".as_bytes())?;
        Ok(())
    }


    fn emit_token(&mut self) -> Result<(), io::Error> {
        // Prints self.tokenizer.current_token() in xml form.

        let ct = self.tokenizer.current_token().unwrap();

        let value = if let TokenType::Symbol = ct.get_token_type() {
            match ct.get_value().as_str() {
                "<" => "&lt;",
                ">" => "&gt;",
                "&" => "&amp;",
                _   => ct.get_value()
            }
        } else {
            ct.get_value()
        };

        let xml_str = format!(
            "<{}> {} </{}>\n",
            ct.get_token_type(), value, ct.get_token_type()
        ); 
        self.vm_writer.write_all(xml_str.as_bytes())?;
        Ok(())
    }

    fn check_token(
        &mut self,
        token_type: TokenType, 
        values: Option<&[&str]>, 
        peek: bool
    ) -> Result<(), io::Error> {
        /*
            Checks current token or next token with the token_type and values (if any).
            When checking current value it advances the tokenizer and emits the token in xml.
         */

        let ct = if peek {
            self.tokenizer.peek().ok_or_else(|| self.compilation_error("There is no next token."))?
        } else {
            self.tokenizer.advance()?;
            self.tokenizer.current_token().ok_or_else(|| self.compilation_error("There is no current token."))?
        };

        let token_value = ct.get_value();
        let ctoken_type  = ct.get_token_type();

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
            )
        }

        if !peek { 
            self.emit_token()? 
        }

        Ok(())
    }


    fn check_type(&mut self, next_token: bool) -> Result<(), io::Error> {
        let ct = if next_token {
            self.tokenizer.peek().ok_or_else(|| self.compilation_error("There is no next token."))?
        } else {
            self.tokenizer.advance()?;
            self.tokenizer.current_token().ok_or_else(|| self.compilation_error("There is no current token."))?
        };
       
        let token_value = ct.get_value();
        let token_type = ct.get_token_type();
       
        if *token_type == TokenType::Identifier {
            if !next_token { self.emit_token()? }
            return Ok(())
        } 

        if *token_type == TokenType::Keyword 
            && ["int", "char", "boolean"].contains(&token_value.as_str()) {

            if !next_token { self.emit_token()? }
            return Ok(())
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
            )
        )
    }
}

