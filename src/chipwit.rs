extern crate rand;
use rand::{thread_rng, Rng};

use crate::fontset::{FONTSET, FONTSET_SIZE, FONTSET_START_ADDRESS};
use crate::rom::Rom;

pub const REGISTER_SIZE: usize = 16;
pub const MEMORY_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 16;
pub const KEYPAD_SIZE: usize = 16;
pub const VIDEO_SIZE: usize = 64 * 32;

pub const START_ADDRESS: usize = 0x200;

/// My understanding of the CHIP-8 structure.
///
/// From what I've gathered the CHIP-8 strucure should contain:
/// - 16 8-bit registers
/// - 4kB 8-bit memory or 4096 bytes of memory
/// - 1 16-bit Index register to store memory addresses
/// - 1 16-bit Program Counter to store the address of the next instruction to execute
/// - 1 8-bit stack to store Program Counter and keep track of the order of execution of calls
/// - 1 8-bit Stack Pointer to keep track of where the last value was place in stack
/// - 1 8-bit delay timer for logic & video timing
/// - 1 8-bit sound timer for sound timing
/// - 16 inputs
/// - 1 64*32 monochrome display
#[derive(Debug, Default)]
pub struct Chipwit {
    pub registers: Vec<u8>,
    pub memory: Vec<u8>,
    pub index_register: u16,
    pub program_counter: u16,
    pub stack: Vec<u8>,
    pub stack_pointer: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub keypad: Vec<u8>,
    pub video: Vec<u32>,

    /// `operation code`/`machine code`/`instruction code`.
    ///
    /// Specifies the operation to be performed
    pub opcode: u16,

    /// `CXNN` instruction sets a random number (Why?)
    random_byte: u8,
}

impl Chipwit {
    pub fn new() -> Self {
        Chipwit {
            registers: Vec::with_capacity(REGISTER_SIZE),
            memory: vec![0; MEMORY_SIZE],
            program_counter: START_ADDRESS as u16,
            stack: Vec::with_capacity(STACK_SIZE),
            keypad: Vec::with_capacity(KEYPAD_SIZE),
            video: Vec::with_capacity(VIDEO_SIZE),
            random_byte: thread_rng().gen(),
            ..Default::default()
        }
    }

    pub fn load_rom(&mut self, path: &str) {
        let rom: Rom = Rom::new(path);

        println!("rom: {:?}", rom);

        let dataset_size = rom.data.len();

        for i in 0..dataset_size {
            self.memory[START_ADDRESS + i] = rom.data[i];
        }
    }

    pub fn load_font(&mut self) {
        for i in 0..FONTSET_SIZE {
            self.memory[FONTSET_START_ADDRESS + i] = FONTSET[i];
        }
    }
}
