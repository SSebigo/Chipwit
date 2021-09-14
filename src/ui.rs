extern crate sdl2;

use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, render::WindowCanvas, EventPump, VideoSubsystem,
};

const WINDOW_TITLE: &str = "Chipwit";
const WINDOW_WIDTH: u32 = 64_u32 * 8_u32;
const WINDOW_HEIGHT: u32 = 32_u32 * 8_u32;

pub struct Ui {
    canvas: WindowCanvas,
    event_pump: EventPump,
    video_subsystem: VideoSubsystem,
}

impl Ui {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().expect("Should provide a sdl2 context");
        let video_subsystem = sdl_context
            .video()
            .expect("Should provide a sdl2 video subsystem");

        let window = video_subsystem
            .window("Chipwit", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .expect("Should provide a sdl2 window");

        let mut canvas = window
            .into_canvas()
            .build()
            .expect("Should provide a sdl2 canvas");

        let event_pump = sdl_context
            .event_pump()
            .expect("Should provide a sdl2 EventPump");

        Self {
            canvas: canvas,
            event_pump: event_pump,
            video_subsystem: video_subsystem,
        }
    }

    pub fn clear_screen(&mut self) {
        self.canvas.clear();
    }

    pub fn draw(&mut self) {
        self.canvas.present();
    }

    pub fn update(&mut self) {}

    pub fn run(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();

        'running: loop {
            self.canvas.clear();

            for event in self.event_pump.poll_iter() {
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

            self.canvas.present();
        }
    }
}
