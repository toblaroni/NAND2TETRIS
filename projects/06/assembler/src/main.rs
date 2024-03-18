use std:: {
    env,
    process
};

mod assembler;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();    

    match args.len() {
        1 => {
            println!("Please specify a source file to be assembled");
            process::exit(-1);
        }

        n if n > 2 => {
            println!("Too many arguments provided. Expected only 1.");
            process::exit(-1);
        }

        _ => {
            println!("Assembling {}", args[1]);
            assembler::assemble(&args[1]);
        }
    }
}