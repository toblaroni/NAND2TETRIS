#![allow(non_snake_case)]

use std::env;
use std::process::exit;

mod compilation_engine;
mod jack_compiler;
mod tokenizer;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let input = if let 2 = args.len() {
        args.get(1).unwrap()
    } else {
        println!("Usage: cargo run -- <path-to-source>");
        exit(-1)
    };

    let c = jack_compiler::JackCompiler::new(input)?;
    
    match c.compile() {
        Ok(_) => println!("Analyzing finished successfully"),
        Err(e) => {
            println!("Error occurred while analyzing source: {}", e);
        }
    }

    Ok(())
}
