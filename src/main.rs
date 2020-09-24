mod build_cmd;
mod command_file;

use crate::build_cmd::*;
use crate::command_file::CommandFileService;
use anyhow::Result;
use std::env;
use std::process;
use std::process::Command;

fn run(args: Vec<String>) -> Result<()> {
    let matches = build_matches(args);
    if let Some("init") = matches.value_of("command") {
        build_config_file()?;
        return Ok(());
    }

    let command_file_service = CommandFileService::new(
        env::var("CONFIG_FILE_KIND").ok(),
        env::var("CONFIG_FILE_PATH").ok(),
    );
    let cmd_list = command_file_service.parse_config_file()?;
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
