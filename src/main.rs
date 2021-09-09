extern crate clap;
use clap::{App, Arg};

mod rom;

fn main() {
    let args = App::new("Chipwit")
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .required(true)
                .takes_value(true)
                .help("Provides a path to a rom file"),
        )
        .get_matches();

    let path = args
        .value_of("path")
        .expect("Provides a path to a rom file");

    let buffer = match rom::get_data(path) {
        Ok(data) => data,
        Err(err) => panic!("Unable to open or read rom: {}", err),
    };

    println!("buffer: {:?}", buffer);
}
