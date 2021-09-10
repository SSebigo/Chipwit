use std::{
    fs::File,
    io::{Read, Result},
    process::exit,
};

#[derive(Debug)]
pub struct Rom {
    pub data: Vec<u8>,
}

impl Rom {
    pub fn new(path: &str) -> Rom {
        let data: Vec<u8> = match Self::read_rom_file(path) {
            Ok(data) => data,
            Err(err) => {
                eprintln!(
                    "Encountered an error at an unrecoverable point! Terminating. Error details: {}",
                    err
                );
                exit(0)
            }
        };

        Self { data: data }
    }

    fn read_rom_file(path: &str) -> Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize);

        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }
}
