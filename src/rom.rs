use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

/// Takes ``Vec<u8>`` data and convert it to ``Vec<String>``
/// Essentially converting ``base 10`` vector to ``hex`` vector.
pub fn convert_rom_data(buffer: Vec<u8>) -> Result<Vec<String>> {
    Ok(buffer.iter().map(|x| format!("{:#X}", x)).collect())
}

/// Returns ROM data as ``Vec<u8>``.
pub fn get_data(path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize);

    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
