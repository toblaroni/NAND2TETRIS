use std::env;

#[allow(non_snake_case)]
mod vm_translator;
mod parser;
mod code_writer;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => vm_translator::vm_translate(args[1].clone()),
        _ => println!("Usage: `cargo run -- <path-to-source-file>")
    }
}
