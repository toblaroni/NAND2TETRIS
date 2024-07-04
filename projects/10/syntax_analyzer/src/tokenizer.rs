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

#[derive(Debug)]
pub enum TokenType {
    Keyword,
    Symbol,
    Identifier,
    IntConst,
    StringConst,
}


#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: String,
}

#[derive(Debug)]
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

        if !self.has_more_tokens {
            return Ok(None)
        }

        self.current_token = self.next_token.take();

        if self.current_line.is_empty() {
            self.get_next_line()?;
        }

        // cleans the self.current_line variable of all comments... (single-line, multi-line and inline comments)
        self.handle_comments()?;

        let c = self.current_line[0];

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
        } else if c.is_numeric() {
            self.get_integer_constant();
        } else if c.is_alphabetic() || c == '_' {
            self.get_identifier_keyword();
        } else {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Encountered illegal character {}.", c)
                )
            )
        }

        Ok(self.current_token.as_ref())
    }

    fn get_next_line(&mut self) -> Result<(), io::Error> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => {
                // EOF
                self.has_more_tokens = false;
                return Ok(());
            },
            Ok(_) => {
                let line = line.trim().to_owned();

                let line = self.remove_inline_comment(line);

                self.current_line = line.chars().collect();
                println!("Current line: {:?}", self.current_line);

            },
            Err(e) => return Err(e)
        }
        Ok(())
    }
    

    fn remove_inline_comment(&self, line: String) -> String {
        if let Some(i) = line.find("//") {
            line[..i].to_owned()
        } else {
            line
        }
    }

    fn handle_comments(&mut self) -> Result<(), io::Error> {
        // NOTE: Inline comments are handled in get_next_line
        if self.current_line.is_empty() {
            self.get_next_line()?;
        } 

        if !self.has_more_tokens() { return Ok(()) }

        self.trim_current_line();
        // For single line we just want to consume the current line and call advance again
        if self.current_line.starts_with(&['/', '/']) {
            println!("Consuming single line comment");
            self.current_line.clear();
            self.advance()?;
        } else if self.current_line.starts_with(&['/', '*']) {
            // For multi-line, keep removing characters until we reach */
            while !self.current_line.starts_with(&['*', '/']) {
                self.current_line.remove(0);
            }

            self.current_line.drain(0..2);  // Remove "*/"
           
            if self.current_line.is_empty() {
                self.advance()?;
            }
            self.trim_current_line();
            self.handle_comments()?;    // Might be multiple multi-line comments one after another
        }

        Ok(())
    }
    
    fn trim_current_line(&mut self) {
        // Thanks gpt
        let start = self.current_line.iter().position(|&c| !c.is_whitespace()).unwrap_or(0);
        let end = self.current_line.iter().rposition(|&c| !c.is_whitespace()).unwrap_or(self.current_line.len()-1);
        self.current_line.drain(..start);
        self.current_line.drain((end+1-start)..);
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
