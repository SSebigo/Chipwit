use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

#[derive(Debug)]
pub struct Rom {
    data: Vec<String>,
}

impl Rom {
    pub fn new(path: &str) -> Rom {
        let buffer: Vec<u8> = Self::get_raw_data(path).expect("Returns ROM raw data");
        let buffer_hex: Vec<String> =
            Self::convert_to_hex(buffer).expect("Returns buffer raw data as hex data");

        Self { data: buffer_hex }
    }

    /// Takes raw data and convert it to hex values.
    fn convert_to_hex(buffer: Vec<u8>) -> Result<Vec<String>> {
        Ok(buffer.iter().map(|x| format!("{:#X}", x)).collect())
    }

    /// Returns ROM data as ``Vec<u8>``.
    fn get_raw_data(path: &str) -> Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize);
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}
