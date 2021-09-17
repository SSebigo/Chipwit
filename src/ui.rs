extern crate sdl2;

use sdl2::{
    event::Event,
    pixels::PixelFormatEnum,
    render::{TextureCreator, WindowCanvas},
    video::WindowContext,
    EventPump,
};

pub struct Ui {
    canvas: WindowCanvas,
    event_pump: EventPump,
    texture_creator: TextureCreator<WindowContext>,
}

impl Ui {
    pub fn new(title: &str, width: u32, height: u32, scale: u32) -> Ui {
        let sdl_context = sdl2::init().expect("Should provide a sdl2 context");
        let video_subsystem = sdl_context
            .video()
            .expect("Should provide a sdl2 video subsystem");

        let window = video_subsystem
            .window(title, width * scale, height * scale)
            .position_centered()
            .build()
            .expect("Should provide a sdl2 window");

        let canvas = window
            .into_canvas()
            .build()
            .expect("Should provide a sdl2 canvas");

        let event_pump = sdl_context
            .event_pump()
            .expect("Should provide a sdl2 EventPump");

        let texture_creator = canvas.texture_creator();

        Self {
            canvas,
            event_pump,
            texture_creator: texture_creator,
        }
    }

    pub fn set_frame_rgb24(&mut self, rgb24: &Vec<Vec<u8>>, width: usize, height: usize) {
        let mut texture = self
            .texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, width as u32, height as u32)
            .expect("Should provide a sdl2 texture");

        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..height {
                    for x in 0..width {
                        let offset = y * pitch + x * 3;

                        buffer[offset] = rgb24[y][x * 3] as u8;
                        buffer[offset + 1] = rgb24[y][x * 3 + 1] as u8;
                        buffer[offset + 2] = rgb24[y][x * 3 + 2] as u8;
                    }
                }
            })
            .expect("Texture with lock success");

        self.canvas.clear();
        self.canvas
            .copy(&texture, None, None)
            .expect("Canvas copy success");
        self.canvas.present();
    }

    pub fn update(&mut self) -> Vec<Event> {
        let mut events: Vec<Event> = Vec::new();

        match self.event_pump.poll_event() {
            Some(event) => {
                events.push(event);
            }
            None => {}
        }

        events
    }
}
