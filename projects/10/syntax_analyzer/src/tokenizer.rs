// Lexer

use std::fs::File;
use std::path::PathBuf;
use std::io::{self, BufRead, BufReader, Read};

pub enum Keyword { Class, Method, Function, Constructor, Int, Boolean, Char, Void, Var, 
                   Static, Field, Let, Do, If, Else, While, Return, True, False, Null, This }

pub enum TokenType { Keyword, Symbol, Identifier, IntConst, StringConst }

pub struct Token {
    token_type: TokenType,
    value: String,
    keyword: Option<Keyword>
}

pub struct Tokenizer {
    reader: BufReader<File>,
    has_more_tokens: bool,
    current_token: Option<Token>, // Value of current token
    next_token: Option<Token>,
    current_line: Vec<char>
}

impl Tokenizer {
    pub fn new(source_file: PathBuf) -> Result<Tokenizer, io::Error> {
        let file = File::open(source_file)?;
        let reader = BufReader::new(file);

        Ok(Tokenizer{
            reader,
            has_more_tokens: true,
            current_token: None,
            next_token: None,
            current_line: Vec::new()
        })
    }


    pub fn advance(&mut self) -> Result<Option<&Token>, io::Error> {
        /*
         *  Returns the next token in the source file.
         *  self.current_token = self.next_token
         *  self.next_token = get_next_token()
         */

        self.current_token = self.next_token.take();


        if self.current_line.is_empty() {
            // This can be its own function
            let mut line = String::new();
            match self.reader.read_line(&mut line) {
                Ok(0) => self.has_more_tokens = false,  // EOF
                Ok(_) => {
                    if line.starts_with("//") || line.is_empty() {
                        self.advance()?;
                    } else {
                        let line = self.remove_inline_comment(line);
                        
                        self.current_line = line.chars().collect();
                    }
                },
                Err(e) => return Err(e)
            }
        }

        // Deduce the next token from self.current_line

        Ok(self.current_token.as_ref())
    }


    fn remove_inline_comment(&self, line: String) -> String {
        if let Some(i) = line.find("//") {
            line[..i].to_owned()
        } else {
            line
        }
    }


    pub fn current_token(&self) -> Option<&Token> {
        self.current_token.as_ref()
    }

    pub fn peek(&self) -> Option<&Token> {
        self.next_token.as_ref()
    }
}
