use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

#[derive(Debug)]
pub struct Rom {
    pub data: Vec<u8>,
}

impl Rom {
    pub fn new(path: &str) -> Rom {
        let data: Vec<u8> = Self::get_raw_data(path).expect("Returns ROM raw data");

        Self { data: data }
    }

    fn get_raw_data(path: &str) -> Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize);

        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }
}
