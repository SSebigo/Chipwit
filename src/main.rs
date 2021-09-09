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

    let buffer: Vec<u8> = match rom::get_data(path) {
        Ok(data) => data,
        Err(err) => panic!("Unable to open or read ROM: {}", err),
    };

    println!("buffer: {:?}", buffer);

    let buffer_hex: Vec<String> = buffer.iter().map(|x| format!("{:#X}", x)).collect();

    println!("buffer_hex: {:?}", buffer_hex);
}
