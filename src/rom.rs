use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn get_data(path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
