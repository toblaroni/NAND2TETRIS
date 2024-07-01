// Lexer

use std::fs::File;
use std::io::BufReader;

pub enum Keyword {
    Class,
    Method,
    Function,
    Constructor,
    Int,
    Boolean,
    Char,
    Void,
    Var,
    Static,
    Field,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    This,
}

pub enum TokenType {
    Keyword,
    Symbol,
    Identifier,
    IntConst,
    StringConst,
}

pub struct Tokenizer {
    pub source_file: BufReader<File>,
    pub has_more_tokens: bool,
    pub token_type: TokenType,         // Returns type of current token
    pub current_token: Option<String>, // Value of current token
    keywords: Vec<String>,             // All the keywords of the jack language
}

impl Tokenizer {
}
