use std::process::exit;

extern crate sdl2;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

#[derive(Debug)]
pub struct Ui {
    name: String,
    width: u32,
    height: u32,
}

impl Ui {
    pub fn new(width: Option<u32>, height: Option<u32>) -> Self {
        Self {
            name: String::from("Chipwit"),
            width: match width {
                Some(width) => width,
                None => 64u32 * 8u32,
            },
            height: match height {
                Some(height) => height,
                None => 32u32 * 8u32,
            },
        }
    }

    pub fn run(&self) -> Result<(), String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = match video_subsystem
            .window(self.name.as_str(), self.width, self.height)
            .position_centered()
            .build()
        {
            Ok(window) => window,
            Err(e) => {
                eprintln!(
                    "Encountered an error at an unrecoverable point! Terminating. Error was: {}",
                    e
                );
                exit(0)
            }
        };

        let mut canvas = match window.into_canvas().build() {
            Ok(canvas) => canvas,
            Err(e) => {
                eprintln!(
                    "Encountered an error at an unrecoverable point! Terminating. Error was: {}",
                    e
                );
                exit(0)
            }
        };

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let mut event_pump = sdl_context.event_pump()?;
        let mut i = 0;

        'running: loop {
            i = (i + 1) % 255;

            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }
                    _ => {}
                }
            }

            canvas.present();
        }

        Ok(())
    }
}
