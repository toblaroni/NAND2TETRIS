use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use crate::compilation_engine::{ArithmeticCommand, VMSegment};

pub struct VMWriter {
    writer: BufWriter<File>,
}

impl VMWriter {
    pub fn new(output_file: File) -> VMWriter {
        let writer = BufWriter::new(output_file);

        VMWriter { writer }
    }

    pub fn write_push_pop(
        &mut self,
        push: bool,
        segment: VMSegment,
        index: u32,
    ) -> Result<(), io::Error> {
        todo!()
    }

    pub fn write_arithmetic(&mut self, command: ArithmeticCommand) -> Result<(), io::Error> {
        todo!()
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
        todo!()
    }

    pub fn write_function(&mut self, label: &str, num_locals: u32) -> Result<(), io::Error> {
        todo!()
    }

    pub fn write_return(&mut self) -> Result<(), io::Error> {
        todo!()
    }

    pub fn write_all(&mut self, bytes: &[u8]) -> Result<(), io::Error> {
        self.writer.write_all(bytes)?;
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), io::Error> {
        self.writer.flush()?;
        Ok(())
    }
}
