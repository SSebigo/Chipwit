use std::{fs::File, io::Read};

#[derive(Debug)]
pub struct Rom {
    pub data: Vec<u8>,
}

impl Rom {
    pub fn new(path: &str) -> Rom {
        let data: Vec<u8> = Self::fetch_data(path);

        Self { data: data }
    }

    fn fetch_data(path: &str) -> Vec<u8> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => panic!("{}", err),
        };

        let metadata = match file.metadata() {
            Ok(metadata) => metadata,
            Err(err) => panic!("{}", err),
        };

        let mut buffer: Vec<u8> = Vec::with_capacity(metadata.len() as usize);

        match file.read_to_end(&mut buffer) {
            Ok(_) => (),
            Err(err) => panic!("{}", err),
        };

        buffer
    }
}
