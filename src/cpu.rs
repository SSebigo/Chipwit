use std::{collections::HashMap, fs::File, io::Read};

const KEYPAD_SIZE: usize = 16;
const MEMORY_SIZE: usize = 4096;
const REGISTER_SIZE: usize = 16;
const STACK_SIZE: usize = 16;

const START_ADDRESS: usize = 0x200;

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

type Instruction = fn();

#[derive(Debug)]
pub struct Cpu {
    current_opcode: u16,
    // 60Hz timers.
    delay_timer: u8,
    instructions: HashMap<u16, Instruction>,
    keypad: [u8; KEYPAD_SIZE],
    memory: [u8; MEMORY_SIZE],
    // Points to the next instruction in memory_ to execute.
    program_counter: u16,
    register_index: u16,
    register: [u16; REGISTER_SIZE],
    sound_timer: u8,
    // Points to the next empty spot in stack_.
    stack_pointer: u16,
    stack: [u8; STACK_SIZE],
}

impl Cpu {
    pub fn new() -> Cpu {
        Self {
            current_opcode: 0,
            memory: [0; MEMORY_SIZE],
            // Program memory begins at 0x200.
            program_counter: START_ADDRESS as u16,
            register: [0; REGISTER_SIZE],
            delay_timer: 0,
            instructions: HashMap::new(),
            keypad: [0; KEYPAD_SIZE],
            register_index: 0,
            sound_timer: 0,
            stack_pointer: 0,
            stack: [0; STACK_SIZE],
        }
    }

    pub fn init(&mut self, rom: &str) {
        for i in 0..FONT_SET.len() {
            self.memory[i + 0x50] = FONT_SET[i];
        }

        let buffer = Self::load_rom(rom);
        for (i, data) in buffer.iter().enumerate() {
            self.memory[i + START_ADDRESS] = *data;
        }
    }

    pub fn run_cycle(&mut self) {}

    fn build_instruction_set(&mut self) {}

    fn load_rom(rom: &str) -> Vec<u8> {
        let mut file = File::open(rom).expect("Should open rom file");

        let metadata = file.metadata().expect("Should read metadata");

        let mut buffer: Vec<u8> = Vec::with_capacity(metadata.len() as usize);

        file.read_to_end(&mut buffer)
            .expect("Should read to end of file");

        buffer
    }

    // pub fn load_data(&mut self, data: &[u8]) {
    //     for (i, data_) in data.iter().enumerate() {
    //         self.ram[START_ADDRESS + i] = *data_;
    //     }

    //     println!("{:?}", self.ram);
    // }

    // pub fn run(&mut self) {
    //     loop {
    //         let nibbles = (
    //             ((self.opcode & 0xF000) >> 12) as u8,
    //             ((self.opcode & 0x0F00) >> 8) as u8,
    //             ((self.opcode & 0x00F0) >> 4) as u8,
    //             (self.opcode & 0x000F) as u8,
    //         );
    //     }
    // }
}
