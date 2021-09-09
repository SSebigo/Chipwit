use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

#[derive(Debug)]
pub struct Rom {
    pub data: Vec<u8>,
}

impl Rom {
    pub fn new(path: &str) -> Rom {
        let data: Vec<u8> = Self::read_rom_file(path).expect("Returns ROM data");

        Self { data: data }
    }

    fn read_rom_file(path: &str) -> Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize);

        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }
}
