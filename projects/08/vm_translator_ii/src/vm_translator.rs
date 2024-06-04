use std::fs;
use std::path::Path;
use std::process;

use crate::parser;
use crate::code_writer;

pub fn vm_translate(input: String) {
    /* ==================================================================
     *  input  -> fileName.vm
     *  output -> fileName.asm
     *  
     *  Logic
     *  - Constructs parser to handle the input file
     *  - Constructs codeWriter to handle the output file
     *  - Marches through the file, parsing each line and generating code.
     * 
     *  Translator is extremely temperamental and will panic at when 
     *  encountering any bug.
     *  Probs better to handle these with Result<> but idk...
     * 
     * ================================================================== */

    let files: Vec<String> = handle_input(&input);
     
    if files.is_empty() {
        translation_error("Couldn't find any .vm files to translate...")
    }
    
    let output_file: String = create_output_file(&input);
    let mut code_writer     = code_writer::CodeWriter::new(&output_file);   // One Code Writer for every input file
    
    code_writer.init();
    for vm_file in files {
        println!("Translating {}...", &vm_file);

        let mut parser = parser::Parser::new(vm_file);
        
        while parser.has_more_commands() {
            parser.advance();  // Update parser.currentCommand
            
            if let Some(command) = parser.get_current_command() {
                // println!("{}", command);
                code_writer.translate_command(command)
            }
        }
    }

    println!("Finished translating all VM files!\nOutput -> {}", output_file);
}


fn handle_input(input: &String) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();

    if input.rfind(".vm").is_some() {
        // Just one vm file
        files.push(input.clone());
    } else {
        // Folder
        // -- Sureleh there's a better way...
        for element in fs::read_dir(input)
                          .unwrap_or_else(|err| translation_error(&err.to_string())) {

                              
            let path = element.unwrap().path();

            if let Some(extension) = path.extension() {
                if extension == "vm" {
                    files.push(path.into_os_string().into_string().unwrap());
                }
            }
        }
    };

    files
}

fn create_output_file(vm_file: &String) -> String {
    if let Some(index) = vm_file.rfind(".vm") {
        vm_file[..index].to_string() + ".asm"
    } else {
        // Create output file from folder
        let path = Path::new(vm_file);

        // Get the value of the last string
        let last_dir = if let Some(last_dir) = path.file_name() {
            if let Some(last_dir_str) = last_dir.to_str() {
                format!("{}.asm", last_dir_str)
            } else {
                translation_error(&format!("Could not deduce output file name from {}", vm_file));
            }
        } else {
            translation_error(&format!("Could not deduce output file name from {}", vm_file));
        };

        let path = path.join(last_dir);
        if let Some(str) = path.to_str() {
            str.to_string()
        } else {
            translation_error(&format!("Couldn't create output file from {}", vm_file))
        }
    }
}




pub fn translation_error(msg: &str) -> ! {
    eprintln!("VM Translation Error: {}", msg);
    process::exit(-1);
}