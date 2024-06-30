use std::env;

mod syntax_analyzer;
mod compilation_engine;
mod tokenizer;

fn main() {
    let args: Vec<String> = env::args.collect();

    match args.len() {
        2 => syntax_analyzer::compile(args[1].clone()),
        _ => println!("Usage: `cargo run -- <path-to-source>")
    }
}
