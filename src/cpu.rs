const RAM_SIZE: usize = 4096;
const REGISTERS_SIZE: usize = 16;

const START_ADDRESS: usize = 0x200;

#[derive(Debug)]
pub struct Cpu {
    opcode: u16,
    pc: usize,
    registers: [u8; REGISTERS_SIZE],
    ram: [u8; RAM_SIZE],
}

impl Cpu {
    pub fn new() -> Cpu {
        Self {
            pc: START_ADDRESS,
            opcode: 0,
            registers: [0; REGISTERS_SIZE],
            ram: [0; RAM_SIZE],
        }
    }

    pub fn load_data(&mut self, data: &[u8]) {
        for (i, data_) in data.iter().enumerate() {
            self.ram[START_ADDRESS + i] = *data_;
        }

        println!("{:?}", self.ram);
    }

    pub fn run(&mut self) {
        loop {
            let nibbles = (
                ((self.opcode & 0xF000) >> 12) as u8,
                ((self.opcode & 0x0F00) >> 8) as u8,
                ((self.opcode & 0x00F0) >> 4) as u8,
                (self.opcode & 0x000F) as u8,
            );
        }
    }
}
