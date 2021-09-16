mod cpu;
mod frame;

use cpu::Cpu;

pub fn main() {
    let mut cpu = Cpu::new();
    cpu.init("roms/Chip8 Picture.ch8");

    println!("{:?}", cpu);

    loop {
        cpu.run_cycle();
    }
}
