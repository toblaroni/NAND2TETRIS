// Recursive top-down parser
use std::io::{self, BufWriter};
use std::path::PathBuf;
use std::fs::File;

use crate::tokenizer::Tokenizer;


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
        let tokenizer = Tokenizer::new(source_file)?;

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


    fn xml_emitter(&mut self, is_terminal: bool) -> Result<(), io::Error> {
        /* 
         *  Print self.tokenizer.current_token() in xml form.
         * 
         *  Non-Terminal:
         *      <xxxx>
         *          Recursive Body of the xxx element
         *      </xxxx>
         * 
         *  Terminal:
         *      <xxxx> terminal </xxxx>
         */

        Ok(())
    }
}