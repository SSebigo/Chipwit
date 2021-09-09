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

    let buffer: Vec<u8> = rom::get_data(path).expect("Returns ROM data as Vec<u8>");

    println!("buffer: {:?}", buffer);

    let buffer_hex: Vec<String> =
        rom::convert_rom_data(buffer).expect("Returns buffer Vec<u8> as Vec<String>");

    println!("buffer_hex: {:?}", buffer_hex);
}
