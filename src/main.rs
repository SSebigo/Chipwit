use std::process::exit;

extern crate clap;
use clap::{App, Arg};

mod cpu;
mod gui;
mod rom;

use cpu::Cpu;
use gui::Gui;
use rom::Rom;

fn main() {
    let args = App::new("Chipwit")
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .required(true)
                .takes_value(true)
                .help("Provide path to your ROM file"),
        )
        .get_matches();

    let path = match args.value_of("path") {
        Some(path) => path,
        None => {
            eprintln!("Encountered an error at an unrecoverable point! Terminating. Error was: Path invalid");
            exit(0)
        }
    };

    let rom: Rom = Rom::new(path);

    println!("{:?}", rom);

    let cpu: Cpu = Cpu::new();

    let gui: Gui = Gui::new(None, None, None, None);
    match gui.run() {
        Ok(()) => {}
        Err(err) => {
            eprintln!(
                "Encountered an error at an unrecoverable point! Terminating. Error was: {}",
                err
            );
            exit(0)
        }
    };
}
