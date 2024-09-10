use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use crate::compilation_engine::VMSegment;

pub struct VMWriter {
    writer: BufWriter<File>,
}

impl VMWriter {
    pub fn new(output_file: File) -> VMWriter {
        let writer = BufWriter::new(output_file);

        VMWriter { writer }
    }

    pub fn write_push(&mut self, segment: VMSegment, index: u32) -> Result<(), io::Error> {
        let seg = match segment {
            VMSegment::Argument => "argument",
            VMSegment::Local => "local",
            VMSegment::Pointer => "pointer",
            VMSegment::Static => "static",
            VMSegment::That => "that",
            VMSegment::This => "this",
            VMSegment::Constant => "constant",
            VMSegment::Temp => "temp",
        };

        self.write_command(&format!("push {} {}", seg, index))
    }

    pub fn write_pop(&mut self, segment: VMSegment, index: u32) -> Result<(), io::Error> {
        let seg = match segment {
            VMSegment::Argument => "argument",
            VMSegment::Local => "local",
            VMSegment::Pointer => "pointer",
            VMSegment::Static => "static",
            VMSegment::That => "that",
            VMSegment::This => "this",
            VMSegment::Constant => "constant",
            VMSegment::Temp => "temp",
        };

        self.write_command(&format!("pop {} {}", seg, index))
    }

    pub fn write_label(&mut self, label: &str) -> Result<(), io::Error> {
        todo!()
    }

    pub fn write_goto(&mut self, label: &str) -> Result<(), io::Error> {
        todo!()
    }

    pub fn write_if(&mut self, label: &str) -> Result<(), io::Error> {
        todo!()
    }

    pub fn write_call(&mut self, label: &str, num_args: u32) -> Result<(), io::Error> {
        self.write_command(&format!("call {} {}", label, num_args))
    }

    pub fn write_function(&mut self, label: &str, num_locals: u32) -> Result<(), io::Error> {
        self.write_command(&format!("function {} {}", label, num_locals))
    }

    pub fn write_alloc(&mut self, size: String) -> Result<(), io::Error> {
        self.write_commands(&[&format!("push {}", size), "call Memory.alloc"])
    }

    pub fn write_commands(&mut self, commands: &[&str]) -> Result<(), io::Error> {
        for command in commands {
            let command = format!("{}\n", command);
            self.writer.write_all(command.as_bytes())?;
        }

        Ok(())
    }

    pub fn write_command(&mut self, command: &str) -> Result<(), io::Error> {
        let command = format!("{}\n", command);
        self.writer.write_all(command.as_bytes())
    }

    pub fn close(&mut self) -> Result<(), io::Error> {
        self.writer.flush()?;
        Ok(())
    }
}
