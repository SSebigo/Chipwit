use std::{fs::File, io::Read};

const KEYPAD_SIZE: usize = 16;
const MEMORY_SIZE: usize = 4096;
const REGISTER_SIZE: usize = 16;
const STACK_SIZE: usize = 16;

const START_ADDRESS: usize = 0x200;
const FONT_START_ADDRESS: usize = 0x50;

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

#[derive(Debug)]
pub struct Cpu {
    current_opcode: u16,
    // 60Hz timers.
    delay_timer: u8,
    keypad: [u8; KEYPAD_SIZE],
    memory: [u8; MEMORY_SIZE],
    // Points to the next instruction in memory_ to execute.
    program_counter: u16,
    register_index: u16,
    register: [u16; REGISTER_SIZE],
    sound_timer: u8,
    // Points to the next empty spot in stack_.
    stack_pointer: u16,
    stack: [u16; STACK_SIZE],
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
            keypad: [0; KEYPAD_SIZE],
            register_index: 0,
            sound_timer: 0,
            stack_pointer: 0,
            stack: [0; STACK_SIZE],
        }
    }

    pub fn init(&mut self, rom: &str) {
        // Load the built-in fontset into 0x050-0x0A0
        for i in 0..FONT_SET.len() {
            self.memory[i + FONT_START_ADDRESS] = FONT_SET[i];
        }

        let buffer = Self::load_rom(rom);
        for (i, data) in buffer.iter().enumerate() {
            self.memory[i + START_ADDRESS] = *data;
        }
    }

    pub fn run_cycle(&mut self) {
        // Read in the big-endian opcode word.
        let opcode =
            (self.memory[START_ADDRESS] as u16) << 8 | (self.memory[START_ADDRESS + 1] as u16);

        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            opcode & 0x000F,
        );

        let nnn: u16 = opcode & 0x0FFF;
        let kk: u16 = opcode & 0x00FF;

        match nibbles {
            // Clear screen
            (0x00, 0x00, 0xE0, 0x00) => self.next(),
            // Return from subroutine
            (0x00, 0x00, 0xE0, 0xE0) => {
                self.program_counter = self.stack[(self.stack_pointer - 1) as usize] as u16;
                self.next()
            }
            (0x10, _, _, _) => self.program_counter = nnn,
            (0x20, _, _, _) => {
                self.stack[(self.stack_pointer + 1) as usize] = self.program_counter;
                self.program_counter = nnn;
            }
            (0x30, _, _, _) => {
                if self.register[nibbles.1 as usize] == kk {
                    self.skip()
                }
                self.next()
            }
            (0x40, _, _, _) => {
                if self.register[nibbles.1 as usize] != kk {
                    self.skip()
                }
                self.next()
            }
            (0x50, _, _, 0x00) => {
                if self.register[nibbles.1 as usize] == self.register[nibbles.2 as usize] {
                    self.skip()
                }
                self.next()
            }
            (0x60, _, _, _) => {
                self.register[nibbles.1 as usize] = kk;
                self.next()
            }
            (0x70, _, _, _) => {
                self.register[nibbles.1 as usize] += kk;
                self.next()
            }
            (0x80, _, _, 0x00) => {
                self.register[nibbles.1 as usize] = self.register[nibbles.2 as usize];
                self.next()
            }
            (0x80, _, _, 0x10) => {
                let x = self.register[nibbles.1 as usize];

                self.register[nibbles.1 as usize] = x | self.register[nibbles.2 as usize];
                self.next()
            }
            (0x80, _, _, 0x20) => {
                let x = self.register[nibbles.1 as usize];

                self.register[nibbles.1 as usize] = x & self.register[nibbles.2 as usize];
                self.next();
            }
            (0x80, _, _, 0x30) => {
                let x = self.register[nibbles.1 as usize];

                self.register[nibbles.1 as usize] = x ^ self.register[nibbles.2 as usize];
                self.next()
            }
            (0x80, _, _, 0x40) => {
                self.register[nibbles.1 as usize] += self.register[nibbles.2 as usize];
                self.register[0xF] = (self.register[nibbles.1 as usize] > 0xFF) as u16;
                self.next()
            }
            (0x80, _, _, 0x50) => {
                self.register[0xF] =
                    (self.register[nibbles.1 as usize] < self.register[nibbles.2 as usize]) as u16;
                self.register[nibbles.1 as usize] -= self.register[nibbles.2 as usize];
                self.next()
            }
            (0x80, _, _, 0x60) => {
                self.register[0xF] = self.register[nibbles.1 as usize] & 1;
                self.register[nibbles.1 as usize] = self.register[nibbles.2 as usize] >> 1;
                self.next()
            }
            (0x80, _, _, 0x70) => {
                self.register[0xF] =
                    (self.register[nibbles.1 as usize] > self.register[nibbles.2 as usize]) as u16;

                let res = self.register[nibbles.2 as usize] - self.register[nibbles.1 as usize];

                self.register[nibbles.1 as usize] = res;
                self.next()
            }
            (0x80, _, _, 0xE0) => {
                self.register[0xF] = 0x80;
                self.register[nibbles.1 as usize] = self.register[nibbles.2 as usize] << 1;
                self.next()
            }
            (0x90, _, _, 0x00) => {}
            (0xA0, _, _, _) => {}
            (0xB0, _, _, _) => {}
            (0xC0, _, _, _) => {}
            (0xD0, _, _, _) => {}
            (0xE0, _, 0x90, 0xE0) => {}
            (0xE0, _, 0xA0, 0x10) => {}
            (0xF0, _, 0x00, 0x70) => {}
            (0xF0, _, 0x00, 0xA0) => {}
            (0xF0, _, 0x10, 0x50) => {}
            (0xF0, _, 0x10, 0x80) => {}
            (0xF0, _, 0x10, 0xE0) => {}
            (0xF0, _, 0x20, 0x90) => {}
            (0xF0, _, 0x30, 0x30) => {}
            (0xF0, _, 0x50, 0x50) => {}
            (0xF0, _, 0x60, 0x50) => {}
            _ => eprint!("Unknown instruction: {:?}", nibbles),
        }

        // TODO: Update sound and delay timers.
    }

    fn load_rom(rom: &str) -> Vec<u8> {
        let mut file = File::open(rom).expect("Should open rom file");

        let metadata = file.metadata().expect("Should read metadata");

        let mut buffer: Vec<u8> = Vec::with_capacity(metadata.len() as usize);

        file.read_to_end(&mut buffer)
            .expect("Should read to end of file");

        buffer
    }

    fn jump_to(&mut self, address: u16) {
        self.program_counter = address;
    }

    /// Go to next instruction.
    ///
    /// e.g. [00, E0, A2, 2A, 60, 0C]
    /// init -> program_counter at 0x00 | instruction = 0x00E0
    /// [next] -> program_counter at A2 | instuction = 0xA22A
    fn next(&mut self) {
        self.program_counter += 2;
    }

    /// Skip nex instruction.
    ///
    /// e.g. [00, E0, A2, 2A, 60, 0C]
    /// init -> program_counter at 00 | instruction = 0x00E0
    /// [skip] -> program_counter at 60 | instuction = 0x600C
    fn skip(&mut self) {
        self.program_counter += 4;
    }
}
