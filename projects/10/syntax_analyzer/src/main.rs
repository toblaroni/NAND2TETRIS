use std::env;
use std::process::exit;

use crate::syntax_analyzer::SyntaxAnalyzer;

mod compilation_engine;
mod syntax_analyzer;
mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if let 2 = args.len() {
        args.get(1).unwrap()
    } else {
        println!("Usage: cargo run -- <path-to-source>");
        exit(-1)
    };

    let syntax_analyzer = SyntaxAnalyzer::new(input);
    match syntax_analyzer {
        Ok(analyzer) => {
            println!("{:?}", analyzer.source_files)
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
