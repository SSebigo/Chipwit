mod cpu;
mod ui;

use cpu::Cpu;

pub fn main() {
    let mut cpu = Cpu::new();
    cpu.init("rom/TETRIS");

    println!("{:?}", cpu);

    loop {
        cpu.run_cycle();
    }
}
