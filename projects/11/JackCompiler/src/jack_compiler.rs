// Top level driver that sets up and invokes other modules
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::compilation_engine::CompilationEngine;

pub struct JackCompiler {
    pub source_files: Vec<PathBuf>,
}

impl JackCompiler {
    pub fn new(input: &str) -> Result<JackCompiler, io::Error> {
        let input_path = Path::new(input);
        let files = Self::handle_input(input_path)?;

        Ok(JackCompiler {
            source_files: files,
        })
    }

    pub fn compile(self) -> Result<(), io::Error> {
        // Loop through each of the source files
        for source_file in self.source_files {
            println!("=========== Compiling {:?} ===========", source_file);
            // Initialise a new tokeniser for each source file
            let mut ce = CompilationEngine::new(source_file)?;
            ce.compile_class()?;
        }

        Ok(())
    }

    fn handle_input(input: &Path) -> Result<Vec<PathBuf>, io::Error> {
        // Method to collect vm files for compilation
        let mut files = Vec::new();

        // Input is either a file <file_name>.vm, or a directory containing multiple jack files
        if Self::is_jack_file(input) {
            files.push(input.to_path_buf())
        } else if input.is_dir() {
            for entry in fs::read_dir(input)? {
                let entry = entry?;
                let path = entry.path();

                if Self::is_jack_file(&path) {
                    files.push(path)
                }
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "No jack files found. Invalid input.",
            ));
        }

        Ok(files)
    }

    fn is_jack_file(path: &Path) -> bool {
        path.is_file() && path.extension().map_or(false, |ext| ext == "jack")
    }
}
