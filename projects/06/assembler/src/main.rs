use std::env;
use std::path;
use std::process;

fn assemble(source_file: string) {

}

fn main() {
    let args: Vec<String> = env::args().collect();    

    if args[1..].len() != 1 {
        println!("Please specify an assembly file to assemble...");
        process::exit(-1);
    }


}