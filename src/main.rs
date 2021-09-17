mod cpu;
mod frame;

use cpu::{Cpu, KEYPAD_SIZE};
use sdl2::{event::Event, keyboard::Keycode};
use ui::Ui;

pub fn main() {
    let width: u32 = 64;
    let height: u32 = 32;

    let mut ui = Ui::new("Chipwit", width, height, 8);

    let mut rgb24: Vec<Vec<u8>> = vec![vec![0; (width * 3) as usize]; height as usize];
    ui.set_frame_rgb24(&rgb24, width as usize, height as usize);

    let mut cpu = Cpu::new();
    cpu.init("roms/KALEID");

    'running: loop {
        let mut num_cycles: u32 = 0;

        for _ in 0..9 {
            cpu.run_cycle();
            num_cycles += 1;
        }

        if num_cycles & 9 == 0 {
            cpu.try_decrement_delay_timer();
            cpu.try_decrement_sound_timer();
        }

        cpu.frame.copy_to_rgb24(&mut rgb24, 241, 196, 15);
        ui.set_frame_rgb24(&rgb24, width as usize, height as usize);

        let events = ui.update();

        let mut keypad: Vec<bool> = vec![false; KEYPAD_SIZE];
        for event in events {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(keycode) => {
                        if keycode == Keycode::Escape {
                            break 'running;
                        }

                        keypad[0] = keycode == Keycode::Num1;
                        keypad[1] = keycode == Keycode::Num2;
                        keypad[2] = keycode == Keycode::Num3;
                        keypad[3] = keycode == Keycode::Num4;
                        keypad[4] = keycode == Keycode::Q;
                        keypad[5] = keycode == Keycode::W;
                        keypad[6] = keycode == Keycode::E;
                        keypad[7] = keycode == Keycode::R;
                        keypad[8] = keycode == Keycode::A;
                        keypad[9] = keycode == Keycode::S;
                        keypad[10] = keycode == Keycode::D;
                        keypad[11] = keycode == Keycode::F;
                        keypad[12] = keycode == Keycode::Z;
                        keypad[13] = keycode == Keycode::X;
                        keypad[14] = keycode == Keycode::C;
                        keypad[15] = keycode == Keycode::V;
                    }
                    None => {}
                },
                _ => {}
            }
        }

        cpu.set_keypad(keypad);
    }
}
