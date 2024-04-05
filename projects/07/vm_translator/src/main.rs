use std::env;

#[allow(non_snake_case)]
mod VMTranslator;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => VMTranslator::vm_translate(args[1].clone()),
        _ => println!("Usage: `cargo run -- <path-to-source-file>")
    }
}