// Lexer
// Reads in the input line by line and then disects each line into tokens

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::fmt;

const SYMBOLS: [char; 20] = [
    '{', '}', '(', ')', '[', ']', '.', ',', ';', '+',
    '-', '*', '/', '&', '|', ',', '<', '>', '=', '~',
];


const KEYWORDS: [&str; 21] = [
    "class", "method", "function", "constructor", "int", "boolean", "char", "void", "var", "static",
    "field", "let", "do", "if", "else", "while", "return", "true", "false", "null", "this",
];


#[derive(Debug, Copy, Clone, PartialEq)]
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
    line_number: u32,
    file_name: String
}

impl Tokenizer {
    pub fn new(source_file: PathBuf) -> Result<Tokenizer, io::Error> {
        let file = File::open(&source_file)?;
        let reader = BufReader::new(file);

        let file_name = source_file.file_name()
                                   .unwrap()
                                   .to_string_lossy()
                                   .into_owned();

        Ok(Tokenizer {
            reader,
            has_more_tokens: true,
            current_token: None,
            next_token: None,
            current_line: Vec::new(),
            line_number: 1,
            file_name
        })
    }

    pub fn advance(&mut self) -> Result<Option<&Token>, io::Error> {
        /*
         *  Returns the next token in the source file.
         *  self.current_token = self.next_token
         *  self.next_token = get_next_token()
         */

        if !self.has_more_tokens { return Ok(None) }

        self.current_token = self.next_token.take();

        if self.current_line.is_empty() {
            self.get_next_line()?;
        }

        if !self.has_more_tokens() { return Ok(None) }

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
            self.get_string_constant()?;
        } else if c.is_numeric() {
            self.get_integer_constant()?;
        } else if c.is_alphabetic() || c == '_' {
            self.get_identifier_keyword()?;
        } else {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "Encountered illegal character {}. Line {} in {}.",
                        c, self.line_number, self.file_name
                    )
                )
            )
        }

        Ok(self.current_token.as_ref())
    }


    fn get_integer_constant(&mut self) -> Result<(), io::Error> {
        let mut value: Vec<char> = Vec::new();     

        let mut c = self.current_line.remove(0);
        while c.is_numeric() {
            if self.current_line.is_empty() {
                return Err(
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("Unexpected end of line while parsing integer. Line {} in {}",
                                self.line_number, self.file_name)
                    )
                )
            }
 
            value.push(c);
            c = self.current_line.remove(0);
        }

        self.current_line.insert(0, c);

        self.next_token = Some(Token {
            value: String::from_iter(value),
            token_type: TokenType::IntConst
        });

        Ok(())
    }


    fn get_identifier_keyword(&mut self) -> Result<(), io::Error> {
        let mut value: Vec<char> = Vec::new();     

        let mut c = self.current_line.remove(0);
        while c.is_alphanumeric() || c == '_' {
            if self.current_line.is_empty() {
                return Err(
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("Unexpected end of line while parsing identifier/keyword. Line {} in {}",
                                self.line_number, self.file_name)
                    )
                )
            }

            value.push(c);
            c = self.current_line.remove(0);
        } 

        // Add the c we just removed from current_line back into current_line
        self.current_line.insert(0, c);

        let value = String::from_iter(value);
        let token_type = if KEYWORDS.contains(&value.as_str()) {TokenType::Keyword} else {TokenType::Identifier};

        self.next_token = Some(Token {
            value,
            token_type
        });

        Ok(())
    }


    fn get_string_constant(&mut self) -> Result<(), io::Error> {
        let mut value: Vec<char> = Vec::new();

        // Set self.current_token to the string constant
        let mut c = self.current_line.remove(0);
        while c != '"' {
            if self.current_line.is_empty() {
                return Err(
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("Unexpected end of line while parsing string constant. Line {} in {}",
                                self.line_number, self.file_name)
                    )
                )
            }

            value.push(c);
            c = self.current_line.remove(0);
        }

        self.next_token = Some(Token {
            value: String::from_iter(value),
            token_type: TokenType::StringConst
        });

        Ok(())
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
                // println!("Current line: {:?}", line);

                self.line_number += 1;
                if line.is_empty() { 
                    // println!("Current line is empty. Fetching new one.");
                    self.get_next_line()?; 
                    return Ok(())
                }

                self.current_line = line.chars().collect();

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
        if self.current_line.starts_with(&['/', '*']) {
            // For multi-line, keep removing characters until we reach */
            while !self.current_line.starts_with(&['*', '/']) {
                if self.current_line.is_empty() {
                    self.get_next_line()?;
                    self.trim_current_line();
                    continue;
                }
                self.current_line.remove(0);
            }

            self.current_line.drain(0..2);  // Remove "*/"
           
            if self.current_line.is_empty() {
                self.get_next_line()?;
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


    pub fn has_more_tokens(&self) -> bool {
        self.has_more_tokens.clone()
    }

    pub fn current_token(&self) -> Option<&Token> {
        self.current_token.as_ref()
    }

    pub fn peek(&self) -> Option<&Token> {
        self.next_token.as_ref()
    }

    pub fn get_line_number(&self) -> u32 {
        self.line_number
    }

    pub fn get_file_name(&self) -> &String {
        &self.file_name
    }
}


impl Token {
    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn get_token_type(&self) -> &TokenType {
        &self.token_type
    }
}


impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Identifier  => write!(f, "identifier"),
            TokenType::Keyword     => write!(f, "keyword"),
            TokenType::IntConst    => write!(f, "integerConstant"),
            TokenType::Symbol      => write!(f, "symbol"),
            TokenType::StringConst => write!(f, "stringConstant"),
        }
    }
}