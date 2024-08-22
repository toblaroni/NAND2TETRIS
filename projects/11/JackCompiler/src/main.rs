#![allow(non_snake_case)]

use std::env;
use std::process::exit;

mod compilation_engine;
mod syntax_analyzer;
mod tokenizer;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let input = if let 2 = args.len() {
        args.get(1).unwrap()
    } else {
        println!("Usage: cargo run -- <path-to-source>");
        exit(-1)
    };

    let s = syntax_analyzer::SyntaxAnalyzer::new(input)?;
    
    match s.analyze() {
        Ok(_) => println!("Analyzing finished successfully"),
        Err(e) => {
            println!("Error occurred while analyzing source: {}", e);
        }
    }

    Ok(())
}
