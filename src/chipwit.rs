pub const REGISTER_SIZE: usize = 16;
pub const MEMORY_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 16;
pub const KEYPAD_SIZE: usize = 16;
pub const VIDEO_SIZE: usize = 64 * 32;

pub const START_ADDRESS: usize = 0x200;

/// More or less accurate representation of the CHIP-8 structure.
#[derive(Debug, Default)]
pub struct Chipwit {
    pub registers: Vec<u8>,
    pub memory: Vec<u8>,
    pub index: u16,
    pub pc: u16,
    pub stack: Vec<u8>,
    pub sp: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub keypad: Vec<u8>,
    pub video: Vec<u32>,
    pub opcode: u16,
}

impl Chipwit {
    pub fn new() -> Self {
        Chipwit {
            registers: Vec::with_capacity(REGISTER_SIZE),
            memory: vec![0; MEMORY_SIZE],
            pc: START_ADDRESS as u16,
            stack: Vec::with_capacity(STACK_SIZE),
            keypad: Vec::with_capacity(KEYPAD_SIZE),
            video: Vec::with_capacity(VIDEO_SIZE),
            ..Default::default()
        }
    }

    pub fn load_rom(&mut self, rom_data: &Vec<u8>) {
        let rom_size = rom_data.len();

        println!("rom_size: {}", rom_size);

        for i in 0..rom_size {
            self.memory[START_ADDRESS + i] = rom_data[i];
        }
    }
}
