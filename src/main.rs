#[macro_use]
extern crate clap;
use clap::App;
use std::process;

fn main() {

    let yaml = load_yaml!("../cli.yml");
    let config = App::from_yaml(yaml).get_matches();
    
    if let Err(e) = nsgn::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    } 
}
