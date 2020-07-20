extern crate clap;

use std::env;
use std::process;

use clap::{Arg, App};

fn main() {

    let config = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("Color")
                .short('c')
                .long("color")
                .takes_value(true)
                .about("Noise color: white, pink, brownian, blue, violet, grey."))
        .arg(Arg::with_name("Interpolation")
                .short('i')
                .long("interpolation")
                .takes_value(true)
                .about("Interpolation type: linear, polynomial, spline."))
        .arg(Arg::with_name("Length")
                .short('l')
                .long("length")
                .takes_value(true)
                .about("Length of generated audio in seconds."))
        .arg(Arg::with_name("Output file")
                .short('o')
                .long("output-file")
                .takes_value(true)
                .about("Output file: [filename.wav]"))        
        .arg(Arg::with_name("Sample Rate")
                .short('s')
                .long("sample-rate")
                .takes_value(true)
                .about("Sample rate: 16k, 44.1k, 48k"))
        .get_matches();
    
    if let Err(e) = nsgn::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    } 
}

