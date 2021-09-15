use std::{fs::File, io::Read};

use rand::Rng;

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
    register: [u8; REGISTER_SIZE],
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
            (0x0, 0x0, 0xE, 0x0) => self.next(),
            // Return from subroutine
            (0x0, 0x0, 0xE, 0xE) => {
                self.program_counter = self.stack[(self.stack_pointer - 1) as usize] as u16;
                self.next()
            }
            (0x1, _, _, _) => self.program_counter = nnn,
            (0x2, _, _, _) => {
                self.stack[(self.stack_pointer + 1) as usize] = self.program_counter;
                self.program_counter = nnn;
            }
            (0x3, _, _, _) => {
                if self.register[nibbles.1 as usize] == (kk as u8) {
                    self.skip()
                }
                self.next()
            }
            (0x4, _, _, _) => {
                if self.register[nibbles.1 as usize] != (kk as u8) {
                    self.skip()
                }
                self.next()
            }
            (0x5, _, _, 0x0) => {
                if self.register[nibbles.1 as usize] == self.register[nibbles.2 as usize] {
                    self.skip()
                }
                self.next()
            }
            (0x6, _, _, _) => {
                self.register[nibbles.1 as usize] = kk as u8;
                self.next()
            }
            (0x7, _, _, _) => {
                self.register[nibbles.1 as usize] += kk as u8;
                self.next()
            }
            (0x8, _, _, 0x0) => {
                self.register[nibbles.1 as usize] = self.register[nibbles.2 as usize];
                self.next()
            }
            (0x8, _, _, 0x1) => {
                let x = self.register[nibbles.1 as usize];

                self.register[nibbles.1 as usize] = x | self.register[nibbles.2 as usize];
                self.next()
            }
            (0x8, _, _, 0x2) => {
                let x = self.register[nibbles.1 as usize];

                self.register[nibbles.1 as usize] = x & self.register[nibbles.2 as usize];
                self.next();
            }
            (0x8, _, _, 0x3) => {
                let x = self.register[nibbles.1 as usize];

                self.register[nibbles.1 as usize] = x ^ self.register[nibbles.2 as usize];
                self.next()
            }
            (0x8, _, _, 0x4) => {
                self.register[nibbles.1 as usize] += self.register[nibbles.2 as usize];
                self.register[0xF] = (self.register[nibbles.1 as usize] as u16 > 0xFF) as u8;
                self.next()
            }
            (0x8, _, _, 0x5) => {
                self.register[0xF] =
                    (self.register[nibbles.1 as usize] < self.register[nibbles.2 as usize]) as u8;
                self.register[nibbles.1 as usize] -= self.register[nibbles.2 as usize];
                self.next()
            }
            (0x8, _, _, 0x6) => {
                self.register[0xF] = self.register[nibbles.1 as usize] & 1;
                self.register[nibbles.1 as usize] = self.register[nibbles.2 as usize] >> 1;
                self.next()
            }
            (0x8, _, _, 0x7) => {
                self.register[0xF] =
                    (self.register[nibbles.1 as usize] > self.register[nibbles.2 as usize]) as u8;

                let res = self.register[nibbles.2 as usize] - self.register[nibbles.1 as usize];

                self.register[nibbles.1 as usize] = res;
                self.next()
            }
            (0x8, _, _, 0xE) => {
                self.register[0xF] = 0x80;
                self.register[nibbles.1 as usize] = self.register[nibbles.2 as usize] << 1;
                self.next()
            }
            (0x9, _, _, 0x0) => {
                if self.register[nibbles.1 as usize] != self.register[nibbles.2 as usize] {
                    self.skip()
                }
                self.next()
            }
            (0xA, _, _, _) => {
                self.register[self.program_counter as usize] = nnn as u8;
                self.next()
            }
            (0xB, _, _, _) => {
                self.stack[(self.stack_pointer + 1) as usize] = self.program_counter;
                self.program_counter = ((nnn as u8) + self.register[0x0]) as u16;
                self.next()
            }
            (0xC, _, _, _) => {
                let mut rng = rand::thread_rng();

                self.register[nibbles.1 as usize] = rng.gen::<u8>() & (kk as u8);
                self.next()
            }
            (0xD, _, _, _) => {
                // Draw sprite at position x/y
                // for i in 0..nibbles.3 {
                //     self.register[0xF] = 01;
                // }
                self.next()
            }
            (0xE, _, 0x9, 0xE) => {
                // Skip if register[x] is key pressed
                self.next()
            }
            (0xE, _, 0xA, 0x1) => {
                // Skip if register[x] is not key pressed
                self.next()
            }
            (0xF, _, 0x0, 0x7) => {
                self.register[nibbles.1 as usize] = self.delay_timer;
                self.next()
            }
            (0xF, _, 0x0, 0xA) => {
                // Wait for keypressed -> store key in register[x]
                self.next()
            }
            (0xF, _, 0x1, 0x5) => {
                self.delay_timer = self.register[nibbles.1 as usize];
                self.next()
            }
            (0xF, _, 0x1, 0x8) => {
                self.sound_timer = self.register[nibbles.1 as usize];
                self.next()
            }
            (0xF, _, 0x1, 0xE) => {
                self.program_counter += self.register[nibbles.1 as usize] as u16;
                self.next()
            }
            (0xF, _, 0x2, 0x9) => {
                // Set I to the memory address of the sprite data corresponding to the hexadecimal digit stored in register VX
                self.next()
            }
            (0xF, _, 0x3, 0x3) => {
                // Set I to the memory address of the sprite data corresponding to the hexadecimal digit stored in register VX
            }
            (0xF, _, 0x5, 0x5) => {
                let end_index = self
                    .register
                    .iter()
                    .position(|&el| el as usize == nibbles.1 as usize)
                    .expect("Should return register index");

                for i in 0..end_index {
                    self.memory[i + (self.program_counter as usize)] = self.register[i];
                }
                self.program_counter = self.program_counter + nibbles.1 as u16 + 1;
                self.next()
            }
            (0xF, _, 0x6, 0x5) => {
                let end_index = self
                    .register
                    .iter()
                    .position(|&el| el as usize == nibbles.1 as usize)
                    .expect("Should return register index");

                for i in 0..end_index {
                    self.register[i] = self.memory[(self.program_counter as usize) + i];
                }
                self.program_counter = self.program_counter + nibbles.1 as u16 + 1;
                self.next()
            }
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
