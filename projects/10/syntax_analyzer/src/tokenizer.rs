// Lexer
// Reads in the input line by line and then disects each line into tokens

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

const SYMBOLS: [char; 19] = [
    '{', '}', '(', ')', '[', ']', '.', ',', ';', '+', '-', '*', '/', '&', ',', '<', '>', '=', '~',
];

const KEYWORDS: [&str; 21] = [
    "Class",
    "Method",
    "Function",
    "Constructor",
    "Int",
    "Boolean",
    "Char",
    "Void",
    "Var",
    "Static",
    "Field",
    "Let",
    "Do",
    "If",
    "Else",
    "While",
    "Return",
    "True",
    "False",
    "Null",
    "This",
];

pub enum TokenType {
    Keyword,
    Symbol,
    Identifier,
    IntConst,
    StringConst,
}

pub struct Token {
    token_type: TokenType,
    value: String,
}

pub struct Tokenizer {
    reader: BufReader<File>,
    has_more_tokens: bool,
    current_token: Option<Token>, // Value of current token
    next_token: Option<Token>,
    current_line: Vec<char>,
}

impl Tokenizer {
    pub fn new(source_file: PathBuf) -> Result<Tokenizer, io::Error> {
        let file = File::open(source_file)?;
        let reader = BufReader::new(file);

        Ok(Tokenizer {
            reader,
            has_more_tokens: true,
            current_token: None,
            next_token: None,
            current_line: Vec::new(),
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
                Ok(0) => {
                    // EOF
                    self.has_more_tokens = false;
                    return Ok(None);
                }
                Ok(_) => {
                    // Handle multi-line comments
                    if line.starts_with("//") || line.is_empty() {
                        self.advance()?;
                    } else {
                        let line = self.remove_inline_comment(line);

                        self.current_line = line.chars().collect();
                    }
                }
                Err(e) => return Err(e),
            }
        }


        /* 
        // Remove whitespace
        let c = if let Some(c) = self.get_non_whitespace() {
            c
        } else {
            // Shouldn't happen since if self.current_line is empty we will have exited or refreshed it earlier
            println!("No more non-whitespace chars found in self.current_line");
            return Ok(None)     
        };


        // Is the first char a symbol?
        if SYMBOLS.contains(&c) {
            self.next_token = Some(Token {
                token_type: TokenType::Symbol,
                value: c.to_string(),
            });
            self.current_line.remove(0);
        } else if c == '"' {
            self.current_line.remove(0);
            self.get_string_constant();
        }

        // String constant
        
        
        // Integer constant
        // Keyword or identifier
        */

        Ok(self.current_token.as_ref())
    }


    fn get_string_constant(&mut self) {
        let mut temp: Vec<char> = Vec::new();

        // Set self.current_token to the string constant
        let mut c = self.current_line.remove(0);
        while c != '"' {
            temp.push(c);
            c = self.current_line.remove(0);
        }

        self.current_token = Some(Token {
            value: String::from_iter(temp),
            token_type: TokenType::StringConst
        });
    }

    fn get_non_whitespace(&mut self) -> Option<char> {
        let first_char = self.current_line.get(0);

        match first_char {
            Some(c) => {
                if c.is_whitespace() {
                    self.current_line.remove(0);
                    self.get_non_whitespace()
                } else {
                    Some(*c)
                }
            },
            None => None        // No current line... 
        }
    }


    fn remove_inline_comment(&self, line: String) -> String {
        if let Some(i) = line.find("//") {
            line[..i].to_owned()
        } else {
            line
        }
    }

    pub fn has_more_tokens(&self) -> bool {
        self.has_more_tokens.clone()
    }

    pub fn current_token(&self) -> Option<&Token> {
        self.current_token.as_ref()
    }

    pub fn peek(&self) -> Option<&Token> {
        self.next_token.as_ref()
    }
}
