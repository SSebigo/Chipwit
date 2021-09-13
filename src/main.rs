use clap::{App, Arg};

mod cpu;
mod rom;
mod ui;

use rom::Rom;

pub fn main() {
    let args = App::new("Chipwit")
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .required(true)
                .takes_value(true)
                .help("Provide path to a ROM file"),
        )
        .get_matches();

    let path = match args.value_of("path") {
        Some(path) => path,
        None => panic!("Invalid path"),
    };

    let rom: Rom = Rom::new(path);

    println!("{:?}", rom);
}
