// Top level driver that sets up and invokes other modules
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct SyntaxAnalyzer {
    pub source_files: Vec<PathBuf>,
}

impl SyntaxAnalyzer {
    pub fn new(input: &str) -> Result<SyntaxAnalyzer, io::Error> {
        let input_path = Path::new(input);
        let files = Self::handle_input(input_path)?;

        Ok(SyntaxAnalyzer {
            source_files: files
        })
    }

    fn handle_input(input: &Path) -> Result<Vec<PathBuf>, io::Error> {
        // Method to collect vm files for compilation
        let mut files = Vec::new();

        // Input is either a file <file_name>.vm, or a directory containing multiple jack files
        if Self::is_jack_file(&input) {
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
