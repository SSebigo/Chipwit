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

/// More or less accurate representation of the CHIP-8 structure.
///
/// Wonder if something like that would work for more complex systems, TODO: try later.
#[derive(Debug, Default)]
pub struct Chipwit {
    /// A register is a dedicated location on a CPU for storage.
    ///
    /// The CHIP-8 has sixteen 8-bit registers, labeled ``V0`` to ``VF``.
    /// Each register can hold any value from ``0x00`` to ``0xFF``.
    pub registers: Vec<u8>,

    /// The CHIP-8 has 4096 bytes of memory, address space from ``0x000`` to ``0xFFF``.
    ///
    /// 16 characters (0 through F) are stored from ``0x050`` to ``0x0A0``.
    /// We need to manually put them in.
    ///
    /// Instructions from the ROM are stored from ``0x200`` to ``0xFFF``.
    /// Everything after instructions space is free to use.
    pub memory: Vec<u8>,
    pub index_register: u16,
    pub program_counter: u16,
    pub stack: Vec<u8>,
    pub stack_pointer: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub keypad: Vec<u8>,
    pub video: Vec<u32>,
    pub opcode: u16,

    /// ``CXNN`` instruction sets a random number (Why?)
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
