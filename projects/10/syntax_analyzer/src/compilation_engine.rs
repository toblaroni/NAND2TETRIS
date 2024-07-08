// Recursive top-down parser
use std::io::{self, BufWriter, ErrorKind, Write};
use std::path::PathBuf;
use std::fs::File;

use crate::tokenizer::{Tokenizer, TokenType};


pub struct CompilationEngine {
    tokenizer: Tokenizer,
    writer: BufWriter<File>,
    output_file_name: String
}

impl CompilationEngine {
    pub fn new(source_file: PathBuf) -> Result<CompilationEngine, io::Error> {
        let mut output = source_file.clone();
        output.set_extension("xml");
        let output_file = File::create(&output)?;
        
        let output_file_name = output.file_name()
                                     .unwrap()
                                     .to_string_lossy()
                                     .into_owned();
        
        let writer = BufWriter::new(output_file);
        let mut tokenizer = Tokenizer::new(source_file)?;

        tokenizer.advance()?;

        Ok(CompilationEngine {
            tokenizer,
            writer,
            output_file_name
        })
    }

    pub fn parse(&mut self) -> Result<(), io::Error> {
        self.compile_class()?;
        Ok(())
    }

    fn compile_class(&mut self) -> Result<(), io::Error> {
        self.writer.write_all("<class>".as_bytes())?;
        self.tokenizer.advance()?;

        if ct.get_token_type() != &TokenType::Keyword || ct.get_value() != "class" {
            return Err(
                self.compilation_error("Expected 'class' keyword.")
            )
        }

        self.emit_xml()?;

        self.tokenizer.advance()?;

        self.emit_xml()?;

        self.writer.write_all("</class>".as_bytes())?;
        Ok(())
    }

    fn compile_class_var_dec(&mut self) -> Result<(), io::Error> {
        Ok(())
    }

    fn compile_subroutine(&mut self) -> Result<(), io::Error> {
        Ok(())
    }

    fn compile_param_list(&mut self) -> Result<(), io::Error> {
        Ok(())
    }

    fn compile_var_dec(&mut self) -> Result<(), io::Error> {
        Ok(())
    }

    fn compile_statements(&mut self) -> Result<(), io::Error> {
        Ok(())
    }

    fn compile_do(&mut self) -> Result<(), io::Error> {
        Ok(())
    }

    fn compile_let(&mut self) -> Result<(), io::Error> {
        Ok(())
    }

    fn compile_while(&mut self) -> Result<(), io::Error> {
        Ok(())
    }

    fn compile_return(&mut self) -> Result<(), io::Error> {
        Ok(())
    }

    fn compile_if(&mut self) -> Result<(), io::Error> {
        Ok(())
    }

    fn compile_expression(&mut self) -> Result<(), io::Error> {
        Ok(())
    }

    fn compile_term(&mut self) -> Result<(), io::Error> {
        Ok(())
    }

    fn compile_expression_list(&mut self) -> Result<(), io::Error> {
        Ok(())
    }


    fn emit_xml(&mut self) -> Result<(), io::Error> {
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

    fn check_token(&self, token_types: &[TokenType], values: Option<&[&str]>) -> Result<(), io::Error> {
        // Check current token against params
        let ct = if let Some(token) = self.tokenizer.current_token() {
            token
        } else {
            return Err(
                self.compilation_error("No tokens found.")
            )
        };


        if values.is_some() {
            let c_value = ct.get_value().as_str();
            if !token_types.contains(ct.get_token_type()) || !values.unwrap().contains(&c_value) {
                return Err(
                    io::Error::new(
                        ErrorKind::InvalidInput,
                        format!(
                            "Expected '{:?}' {:?}. Line {} in file {}.",
                            values.unwrap(),                    // It should be [<value1>, <value2>, ...]
                            token_types,                        // Same for this
                            self.tokenizer.get_line_number(),
                            self.tokenizer.get_file_name()
                        )
                    )
                )
            }
        } else {
            if !token_types.contains(ct.get_token_type()) {
                return Err(
                    io::Error::new(
                        ErrorKind::InvalidInput,
                        format!(
                            "Expected {:?}. Line {} in file {}.",
                            token_types,
                            self.tokenizer.get_line_number(),
                            self.tokenizer.get_file_name()
                        )
                    )
                )
            }
        }

        Ok(())
    }

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

