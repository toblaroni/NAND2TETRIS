// Recursive top-down parser
use std::io::{self, BufWriter, ErrorKind, Write};
use std::path::PathBuf;
use std::fs::File;

use crate::tokenizer::{Tokenizer, TokenType};


pub struct CompilationEngine {
    tokenizer: Tokenizer,
    writer: BufWriter<File>
}

impl CompilationEngine {
    pub fn new(source_file: PathBuf) -> Result<CompilationEngine, io::Error> {
        let mut output = source_file.clone();
        output.set_extension("xml");
        let output_file = File::create(&output)?;
        
        let writer = BufWriter::new(output_file);
        let mut tokenizer = Tokenizer::new(source_file)?;

        tokenizer.advance()?;

        Ok(CompilationEngine {
            tokenizer,
            writer
        })
    }

    pub fn parse(&mut self) -> Result<(), io::Error> {
        self.compile_class()?;
        Ok(())
    }

    fn compile_class(&mut self) -> Result<(), io::Error> {
        self.writer.write_all("<class>\n".as_bytes())?;

        self.check_token(TokenType::Keyword, Some(&["class"]), false)?;     // Class keyword

        self.check_token(TokenType::Identifier, None, false)?;               // Class name

        self.check_token(TokenType::Symbol, Some(&["{"]), false)?;

        // 0 or more
        loop {
            match self.check_token(TokenType::Keyword, Some(&["static", "field"]), true) {
                Ok(()) => self.compile_class_var_dec()?,
                Err(_) => break
            }
        }

        // 0 or more
        loop {
            match self.check_token(TokenType::Keyword, Some(&["constructor", "method", "function"]), true) {
                Ok(()) => self.compile_subroutine()?,
                Err(_) => break
            }
        }

        self.check_token(TokenType::Symbol, Some(&["}"]), false)?;
       
        self.writer.write_all("</class>\n".as_bytes())?;
        Ok(())
    }

    fn compile_class_var_dec(&mut self) -> Result<(), io::Error> {
        self.writer.write_all("<classVarDec>\n".as_bytes())?;
        self.check_token(TokenType::Keyword, Some(&["static", "field"]), false)?;

        self.check_type(false)?;

        self.check_token(TokenType::Identifier, None, false)?;

        loop {
            match self.check_token(TokenType::Symbol, Some(&[","]), true) {
                Ok(()) => {
                    self.tokenizer.advance()?;
                    self.emit_token()?;         // Emit the , symbol
                    self.check_token(TokenType::Identifier, None, false)?;   // Consume and emit identifier
                }
                Err(_) => break
            }
        }

        self.check_token(TokenType::Symbol, Some(&[";"]), false)?;

        self.writer.write_all("</classVarDec>\n".as_bytes())?;
        Ok(())
    }

    fn compile_subroutine(&mut self) -> Result<(), io::Error> {
        self.writer.write_all("<subroutineDec>\n".as_bytes())?;
        self.check_token(TokenType::Keyword, Some(&["constructor", "function", "method"]), false)?;

        // ('void' | type)
        match self.check_token(TokenType::Keyword, Some(&["void"]), true) {
            Ok(()) => {
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

        self.writer.write_all("</subroutineDec>\n".as_bytes())?;
        Ok(())
    }

    fn compile_param_list(&mut self) -> Result<(), io::Error> {
        self.writer.write_all("<parameterList>\n".as_bytes())?;

        match self.check_type(true) {
            Ok(()) => {},
            Err(_) => {
                self.writer.write_all("</parameterList>\n".as_bytes())?;
                return Ok(())
            }
        };

        // Consume type
        self.tokenizer.advance()?;
        self.emit_token()?;

        self.check_token(TokenType::Identifier, None, false)?;

        loop {
            match self.check_token(TokenType::Symbol, Some(&[","]), true) {
                Ok(()) => {
                    // consume ','
                    self.tokenizer.advance()?;
                    self.emit_token()?;

                    self.check_type(false)?;
                    self.check_token(TokenType::Identifier, None, false)?;
                }
                Err(_) => break
            }
        }

        self.writer.write_all("</parameterList>\n".as_bytes())?;
        Ok(())
    }

    fn compile_subroutine_body(&mut self) -> Result<(), io::Error> {
        self.writer.write_all("<subroutineBody>\n".as_bytes())?;
        self.check_token(TokenType::Symbol, Some(&["{"]), false)?;

        loop {
            match self.check_token(TokenType::Keyword, Some(&["var"]), true) {
                Ok(()) => self.compile_var_dec()?,
                Err(_) => break
            }
        }

        self.compile_statements()?;

        self.check_token(TokenType::Symbol, Some(&["}"]), false)?;
        self.writer.write_all("</subroutineBody>\n".as_bytes())?;
        Ok(())
    }

    fn compile_var_dec(&mut self) -> Result<(), io::Error> {
        self.writer.write_all("<varDec>\n".as_bytes())?;

        self.check_token(TokenType::Keyword, Some(&["var"]), false)?;
        self.check_type(false)?;
        self.check_token(TokenType::Identifier, None, false)?;

        loop {
            match self.check_token(TokenType::Symbol, Some(&[","]), true) {
                Ok(()) => {
                    self.tokenizer.advance()?;
                    self.emit_token()?;

                    self.check_token(TokenType::Identifier, None, false)?;
                }
                Err(_) => break
            }
        }

        self.check_token(TokenType::Symbol, Some(&[";"]), false)?;

        self.writer.write_all("</varDec>\n".as_bytes())?;
        Ok(())
    }

    fn compile_statements(&mut self) -> Result<(), io::Error> {
        self.writer.write_all("<statements>\n".as_bytes())?;

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

        self.writer.write_all("</statements>\n".as_bytes())?;
        Ok(())
    }

    fn compile_do(&mut self) -> Result<(), io::Error> {
        self.writer.write_all("<doStatement>\n".as_bytes())?;
        
        // Can just advance and emit since we know 'do' must be next
        self.tokenizer.advance()?; 
        self.emit_token()?;
        
        self.check_token(TokenType::Identifier, None, false)?;

        if let Ok(()) = self.check_token(TokenType::Symbol, Some(&["."]), true) {
            // .subroutineName
            self.tokenizer.advance()?;
            self.emit_token()?;
            self.check_token(TokenType::Identifier, None, false)?;
        }

        self.check_token(TokenType::Symbol, Some(&["("]), false)?;
        self.compile_expression_list()?;
        self.check_token(TokenType::Symbol, Some(&[")"]), false)?;
        self.check_token(TokenType::Symbol, Some(&[";"]), false)?;

        self.writer.write_all("</doStatement>\n".as_bytes())?;
        Ok(())
    }

    fn compile_let(&mut self) -> Result<(), io::Error> {
        self.writer.write_all("<letStatement>\n".as_bytes())?;

        self.check_token(TokenType::Keyword, Some(&["let"]), false)?;
        self.check_token(TokenType::Identifier, None, false)?;

        if let Ok(()) = self.check_token(TokenType::Symbol, Some(&["["]), true) {
            self.tokenizer.advance()?;
            self.emit_token()?;

            self.compile_expression()?;

            self.check_token(TokenType::Symbol, Some(&["]"]), false)?;
        }

        self.check_token(TokenType::Symbol, Some(&["="]), false)?;
        self.compile_expression()?;
        self.check_token(TokenType::Symbol, Some(&[";"]), false)?;

        self.writer.write_all("</letStatement>\n".as_bytes())?;
        Ok(())
    }

    fn compile_while(&mut self) -> Result<(), io::Error> {
        self.writer.write_all("<whileStatement>\n".as_bytes())?;
        self.check_token(TokenType::Keyword, Some(&["while"]), false)?;

        self.check_token(TokenType::Symbol, Some(&["("]), false)?;
        self.compile_expression()?;        
        self.check_token(TokenType::Symbol, Some(&[")"]), false)?;

        self.check_token(TokenType::Symbol, Some(&["{"]), false)?;
        self.compile_statements()?;
        self.check_token(TokenType::Symbol, Some(&["}"]), false)?;

        self.writer.write_all("</whileStatement>\n".as_bytes())?;
        Ok(())
    }

    
    fn compile_return(&mut self) -> Result<(), io::Error> {
        self.writer.write_all("<returnStatement>\n".as_bytes())?;

        self.check_token(TokenType::Keyword, Some(&["return"]), false)?;

        if let Err(_) = self.check_token(TokenType::Symbol, Some(&[";"]), true) {
            self.compile_expression()?;
        }

        self.check_token(TokenType::Symbol, Some(&[";"]), false)?;
        self.writer.write_all("</returnStatement>\n".as_bytes())?;
        Ok(())
    }


    fn compile_if(&mut self) -> Result<(), io::Error> {
        self.writer.write_all("<ifStatement>\n".as_bytes())?;
        self.check_token(TokenType::Keyword, Some(&["if"]), false)?;

        self.check_token(TokenType::Symbol, Some(&["("]), false)?;
        self.compile_expression()?;
        self.check_token(TokenType::Symbol, Some(&[")"]), false)?;

        self.check_token(TokenType::Symbol, Some(&["{"]), false)?;
        self.compile_statements()?;
        self.check_token(TokenType::Symbol, Some(&["}"]), false)?;

        if let Ok(()) = self.check_token(TokenType::Keyword, Some(&["else"]), true) {
            self.tokenizer.advance()?;
            self.emit_token()?;
            self.check_token(TokenType::Symbol, Some(&["{"]), false)?;
            self.compile_statements()?;
            self.check_token(TokenType::Symbol, Some(&["}"]), false)?;
        }

        self.writer.write_all("</ifStatement>\n".as_bytes())?;
        Ok(())
    }


    fn compile_expression_list(&mut self) -> Result<(), io::Error> {
        self.writer.write_all("<expressionList>\n".as_bytes())?;

        if let Ok(()) = self.check_token(TokenType::Symbol, Some(&[")"]), true) {
            self.writer.write_all("</expressionList>\n".as_bytes())?;
            return Ok(())
        }

        self.compile_expression()?;

        while let Ok(()) = self.check_token(TokenType::Symbol, Some(&[","]), true) {
            self.tokenizer.advance()?;
            self.emit_token()?;
            self.compile_expression()?;
        }

        self.writer.write_all("</expressionList>\n".as_bytes())?;
        Ok(())
    }


    fn compile_expression(&mut self) -> Result<(), io::Error> {
        self.writer.write_all("<expression>\n".as_bytes())?;
        self.compile_term()?;

        let ops = &["+", "-", "*", "/", "&", "|", "<", ">", "="];

        // (op term)*
        while let Ok(_) = self.check_token(TokenType::Symbol, Some(ops), true) {
            self.tokenizer.advance()?;
            self.emit_token()?;
            self.compile_term()?;
        }

        self.writer.write_all("</expression>\n".as_bytes())?;
        Ok(())
    }

    fn compile_term(&mut self) -> Result<(), io::Error> {
        self.writer.write_all("<term>\n".as_bytes())?;

        if let Some(t) = self.tokenizer.peek() {
            match t.get_token_type() {
                TokenType::IntConst    => self.check_token(TokenType::IntConst, None, false)?,
                TokenType::StringConst => self.check_token(TokenType::StringConst, None, false)?,
                TokenType::Keyword     => self.check_token(TokenType::Keyword, Some(&["true", "false", "null", "this"]), false)?,
                TokenType::Identifier  => self.handle_term_id()?,
                TokenType::Symbol      => self.handle_term_symbol()?
            }
        }

        self.writer.write_all("</term>\n".as_bytes())?;
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


    fn emit_token(&mut self) -> Result<(), io::Error> {
        // Prints self.tokenizer.current_token() in xml form.

        let ct = self.tokenizer.current_token().unwrap();

        let value = if let TokenType::Symbol = ct.get_token_type() {
            match ct.get_value().as_str() {
                "<" => "&lt;",
                ">" => "&gt;",
                "&" => "&amp;",
                _   => &ct.get_value()
            }
        } else {
            ct.get_value()
        };

        let xml_str = format!(
            "<{}> {} </{}>\n",
            ct.get_token_type(), value, ct.get_token_type()
        ); 
        self.writer.write_all(xml_str.as_bytes())?;
        Ok(())
    }

    fn check_token(
        &mut self,
        token_type: TokenType, 
        values: Option<&[&str]>, 
        next_token: bool
    ) -> Result<(), io::Error> {
        /*
            Checks current token or next token with the token_type and values (if any).
            When checking current value it advances the tokenizer and emits the token in xml.
         */

        let ct = if next_token {
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

        if !next_token { self.emit_token()? }

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
        } else if *token_type == TokenType::Keyword  {
            if ["int", "char", "boolean"].contains(&token_value.as_str()) {
                if !next_token { self.emit_token()? }
                return Ok(())
            }
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

