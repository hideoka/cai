use clap::{App, Arg};
use std::env;
use std::error::Error;
use std::process;
use std::process::Command;

#[macro_use]
extern crate clap;

fn run() -> Result<(), Box<dyn Error>> {
    let args = env::args_os();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("command").index(1).required(true))
        .get_matches_from(args);

    match matches.value_of("command") {
        Some(command) => {
            Command::new("bash").arg("-c").arg(command).spawn()?;
            Ok(())
        }
        None => unimplemented!(),
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
