use clap::{App, Arg};
use std::env;
use std::error::Error;
use std::process;
use std::process::Command;

#[macro_use]
extern crate clap;

struct Cmd<'a> {
    command: &'a str,
    args: Option<Vec<&'a str>>,
}

impl<'a> Cmd<'a> {
    fn new(command: &'a str, args: Option<Vec<&'a str>>) -> Cmd<'a> {
        Cmd { command, args }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = env::args_os();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("command").index(1).required(true))
        .arg(Arg::with_name("args").min_values(0))
        .get_matches_from(args);

    let command = matches.value_of("command").unwrap();
    let args = matches.values_of("args").map(|a| a.collect::<Vec<_>>());
    let cmd = Cmd::new(command, args);

    if let Some(args) = cmd.args {
        Command::new("bash")
            .arg("-c")
            .arg(format!("{:?} {:?}", cmd.command, args.join(" ")))
            .spawn()?;
    } else {
        Command::new("bash").arg("-c").arg(cmd.command).spawn()?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
