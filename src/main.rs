mod cpu;
mod frame;
mod ui;

use cpu::Cpu;

pub fn main() {
    let mut cpu = Cpu::new();
    cpu.init("roms/PUZZLE");

    println!("{:?}", cpu);

    loop {
        cpu.run_cycle();
    }
}
