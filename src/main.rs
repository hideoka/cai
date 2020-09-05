use anyhow::Result;
use cai::{build_cinfig_file, build_cmd, build_matches, parse_config_file};
use std::env;
use std::process;
use std::process::Command;

fn run(args: Vec<String>) -> Result<()> {
    let matches = build_matches(args);
    if let Some("init") = matches.value_of("command") {
        build_cinfig_file()?;
        return Ok(());
    }

    let cmd_list = parse_config_file("./cai_config.json")?;
    let cmd = build_cmd(matches, cmd_list)?;
    let mut child = Command::new("bash").arg("-c").arg(cmd).spawn()?;
    child.wait()?;
    Ok(())
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if let Err(e) = run(args) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
