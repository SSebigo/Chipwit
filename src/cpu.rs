const RAM_SIZE: u16 = 4096;

#[derive(Debug)]
pub struct Cpu {
    pub ram: Vec<u8>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Self {
            ram: vec![0; RAM_SIZE as usize],
        }
    }
}
