use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

#[derive(Debug)]
pub struct Rom {
    data: Vec<String>,
}

impl Rom {
    pub fn new(path: &str) -> Rom {
        let buffer: Vec<u8> = Self::get_data(path).expect("Returns ROM data as Vec<u8>");
        let buffer_hex: Vec<String> =
            Self::convert_rom_data(&buffer).expect("Returns buffer Vec<u8> as Vec<String>");

        Self { data: buffer_hex }
    }

    /// Takes ``Vec<u8>`` data and convert it to ``Vec<String>``
    /// Essentially converting ``base 10`` vector to ``hex`` vector.
    fn convert_rom_data(buffer: &Vec<u8>) -> Result<Vec<String>> {
        Ok(buffer.iter().map(|x| format!("{:#X}", x)).collect())
    }

    /// Returns ROM data as ``Vec<u8>``.
    fn get_data(path: &str) -> Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize);
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}
