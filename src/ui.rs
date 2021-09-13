extern crate sdl2;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

const GAME_MODE_WIDTH: u32 = 64_u32 * 8_u32;
const GAME_MODE_HEIGHT: u32 = 32_u32 * 8_u32;

#[derive(Debug)]
pub struct Ui<'a> {
    title: &'a str,
    width: u32,
    height: u32,
}

impl<'a> Ui<'a> {
    pub fn new(width: Option<u32>, height: Option<u32>) -> Self {
        Self {
            title: "Chipwit",
            width: match width {
                Some(width) => width,
                None => GAME_MODE_WIDTH,
            },
            height: match height {
                Some(height) => height,
                None => GAME_MODE_HEIGHT,
            },
        }
    }

    pub fn run(&self) {
        let sdl_context = match sdl2::init() {
            Ok(context) => context,
            Err(err) => panic!("{}", err),
        };

        let video_subsystem = match sdl_context.video() {
            Ok(video) => video,
            Err(err) => panic!("{}", err),
        };

        let window = match video_subsystem
            .window(self.title, GAME_MODE_WIDTH, GAME_MODE_HEIGHT)
            .position_centered()
            .build()
        {
            Ok(window) => window,
            Err(err) => panic!("{}", err),
        };

        let mut canvas = match window.into_canvas().build() {
            Ok(canvas) => canvas,
            Err(err) => panic!("{}", err),
        };

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let mut event_pump = match sdl_context.event_pump() {
            Ok(event_pump) => event_pump,
            Err(err) => panic!("{}", err),
        };

        'running: loop {
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
    }
}
