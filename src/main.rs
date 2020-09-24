mod build_cmd;
mod command_file;

use crate::build_cmd::*;
use crate::command_file::CommandFileService;
use anyhow::Result;
use std::env;
use std::process;
use std::process::Command;

#[derive(Debug)]
pub struct EnvConfig {
    file_kind: Option<String>,
    file_path: Option<String>,
}

fn run(args: Vec<String>, env_config: EnvConfig) -> Result<()> {
    let matches = build_matches(args);
    if let Some("init") = matches.value_of("command") {
        build_config_file()?;
        return Ok(());
    }

    let command_file_service = CommandFileService::new(env_config);
    let cmd_list = command_file_service.parse_config_file()?;
    let cmd = Cmd::new(&matches)?.build_cmd(cmd_list)?;
    let mut child = Command::new("bash").arg("-c").arg(cmd).spawn()?;

    child.wait()?;
    Ok(())
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let env_config = EnvConfig {
        file_kind: env::var("CONFIG_FILE_KIND").ok(),
        file_path: env::var("CONFIG_FILE_PATH").ok(),
    };

    if let Err(e) = run(args, env_config) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
