use std::{fs::File, io::Read};

use rand::Rng;

use crate::frame;
use frame::Frame;

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
    frame: Frame,
    i: u16,
    keypad: [bool; KEYPAD_SIZE],
    memory: [u8; MEMORY_SIZE],
    // Points to the next instruction in memory_ to execute.
    pc: u16,
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
            delay_timer: 0,
            frame: Frame::new(),
            i: 0,
            keypad: [false; KEYPAD_SIZE],
            memory: [0; MEMORY_SIZE],
            // Program memory begins at 0x200.
            pc: START_ADDRESS as u16,
            register: [0; REGISTER_SIZE],
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
        let pc = self.pc as usize;

        // Read in the big-endian opcode word.
        let opcode = (self.memory[pc] as u16) << 8 | (self.memory[pc + 1] as u16);

        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            opcode & 0x000F,
        );

        let nnn: u16 = opcode & 0x0FFF;
        let kk: u16 = opcode & 0x00FF;

        println!("{:#06x}", opcode);

        match nibbles {
            // Clear screen
            (0x0, 0x0, 0xE, 0x0) => {
                self.frame.set_all(0);
                self.next()
            }
            // Return from subroutine
            (0x0, 0x0, 0xE, 0xE) => {
                self.pc = self.stack[(self.stack_pointer - 1) as usize] as u16;
                self.next()
            }
            (0x1, _, _, _) => self.pc = nnn,
            (0x2, _, _, _) => {
                self.stack[(self.stack_pointer - 1) as usize] = self.pc;
                self.pc = nnn;
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
                self.register[nibbles.1 as usize] |= self.register[nibbles.2 as usize];
                self.next()
            }
            (0x8, _, _, 0x2) => {
                self.register[nibbles.1 as usize] &= self.register[nibbles.2 as usize];
                self.next();
            }
            (0x8, _, _, 0x3) => {
                self.register[nibbles.1 as usize] ^= self.register[nibbles.2 as usize];
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
                self.register[0xF] = (self.register[nibbles.1 as usize] > 0x80) as u8;
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
                self.i = nnn;
                self.next()
            }
            (0xB, _, _, _) => {
                self.pc = ((nnn as u8) + self.register[0x0]) as u16;
            }
            (0xC, _, _, _) => {
                let mut rng = rand::thread_rng();

                self.register[nibbles.1 as usize] = rng.gen::<u8>() & (kk as u8);
                self.next()
            }
            (0xD, _, _, _) => {
                self.register[0x0f] = 0;

                for byte in 0..nibbles.3 {
                    let r = (self.register[nibbles.2 as usize] as u16 + byte)
                        % frame::FRAME_HEIGHT as u16;

                    for bit in 0..8 {
                        let c =
                            (self.register[nibbles.1 as usize] + bit) % frame::FRAME_WIDTH as u8;
                        let sprite =
                            (self.memory[(self.i as usize) + (byte as usize)] >> (7 - bit)) & 1;
                        self.register[0xF] |= sprite & self.frame.at(c as usize, r as usize);
                        self.frame.set_one(c as usize, r as usize, sprite);
                    }
                }
                self.next()
            }
            (0xE, _, 0x9, 0xE) => {
                let key = self.register[nibbles.1 as usize];

                if self.keypad[key as usize] {
                    self.skip()
                }
                self.next()
            }
            (0xE, _, 0xA, 0x1) => {
                let key = self.register[nibbles.1 as usize];

                if !self.keypad[key as usize] {
                    self.skip()
                }
                self.next()
            }
            (0xF, _, 0x0, 0x7) => {
                self.register[nibbles.1 as usize] = self.delay_timer;
                self.next()
            }
            (0xF, _, 0x0, 0xA) => {
                unimplemented!();
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
                self.i += self.register[nibbles.1 as usize] as u16;
                self.next()
            }
            (0xF, _, 0x2, 0x9) => {
                let digit = self.register[nibbles.1 as usize];
                self.i = FONT_START_ADDRESS as u16 + (5 * digit) as u16;
                self.next()
            }
            (0xF, _, 0x3, 0x3) => {
                let val = self.register[nibbles.1 as usize];
                let val_hundreds = val / 100;
                let val_tens = (val / 10) % 10;
                let val_ones = (val % 100) % 10;

                self.memory[self.i as usize] = val_hundreds;
                self.memory[self.i as usize + 1] = val_tens;
                self.memory[self.i as usize] = val_ones;

                self.next()
            }
            (0xF, _, 0x5, 0x5) => {
                for i in 0..nibbles.1 {
                    self.memory[(i + self.i) as usize] = self.register[i as usize];
                }
                self.i = self.i + nibbles.1 as u16 + 1;
                self.next()
            }
            (0xF, _, 0x6, 0x5) => {
                for i in 0..nibbles.1 {
                    self.register[i as usize] = self.memory[(self.i + i) as usize];
                }
                self.i = self.i + nibbles.1 as u16 + 1;
                self.next()
            }
            _ => eprint!("Unknown instruction: {:?}", nibbles),
        }

        self.frame.draw_to_stdout();

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
        self.pc += 2;
    }

    /// Skip nex instruction.
    ///
    /// e.g. [00, E0, A2, 2A, 60, 0C]
    /// init -> program_counter at 00 | instruction = 0x00E0
    /// [skip] -> program_counter at 60 | instuction = 0x600C
    fn skip(&mut self) {
        self.pc += 4;
    }
}
