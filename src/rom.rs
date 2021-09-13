use std::{
    fs::File,
    io::{self, Read},
};

#[derive(Debug)]
pub struct Rom {
    data: Vec<u8>,
}

impl Rom {
    pub fn new(path: &str) -> Rom {
        let data: Vec<u8> = match Self::fetch_data(path) {
            Ok(data) => data,
            Err(err) => panic!("{}", err),
        };

        Self { data: data }
    }

    fn fetch_data(path: &str) -> io::Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize);

        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }
}
