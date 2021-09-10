use std::process::exit;

extern crate druid;
extern crate sdl2;

use druid::{widget::Label, AppLauncher, PlatformError, Widget, WindowDesc};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

#[derive(Debug)]
pub struct Gui {
    client_width: u32,
    client_height: u32,
    game_mode_width: u32,
    game_mode_height: u32,
    name: String,
}

impl Gui {
    pub fn new(
        client_width: Option<u32>,
        client_height: Option<u32>,
        game_mode_width: Option<u32>,
        game_mode_height: Option<u32>,
    ) -> Self {
        Self {
            name: String::from("Chipwit"),
            client_width: match client_width {
                Some(width) => width,
                None => 1280u32,
            },
            client_height: match client_height {
                Some(height) => height,
                None => 720u32,
            },
            game_mode_width: match game_mode_width {
                Some(width) => width,
                None => 64u32 * 8u32,
            },
            game_mode_height: match game_mode_height {
                Some(height) => height,
                None => 32u32 * 8u32,
            },
        }
    }

    fn build_ui(&self) -> impl Widget<()> {
        Label::new("Hello world")
    }

    pub fn run(&self) -> Result<(), PlatformError> {
        let window = WindowDesc::new(self.build_ui())
            .title(self.name.as_str())
            .window_size((f64::from(self.client_width), f64::from(self.client_height)));

        AppLauncher::with_window(window).launch(())?;

        Ok(())
    }

    fn run_game_mode(&self) -> Result<(), String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = match video_subsystem
            .window(
                self.name.as_str(),
                self.game_mode_width,
                self.game_mode_height,
            )
            .position_centered()
            .build()
        {
            Ok(window) => window,
            Err(err) => {
                eprintln!(
                    "Encountered an error at an unrecoverable point! Terminating. Error was: {}",
                    err
                );
                exit(0)
            }
        };

        let mut canvas = match window.into_canvas().build() {
            Ok(canvas) => canvas,
            Err(err) => {
                eprintln!(
                    "Encountered an error at an unrecoverable point! Terminating. Error was: {}",
                    err
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
