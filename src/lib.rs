use anyhow::{anyhow, Context, Result};
use clap::{crate_authors, crate_name, crate_version, App, Arg, ArgMatches};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn build_cinfig_file() -> Result<()> {
    let mut file = File::create("cai_config.json")?;
    let content = "{\n\n}";
    file.write_all(content.as_bytes())?;
    Ok(())
}

struct Cmd<'a> {
    command: &'a str,
    args: Option<Vec<&'a str>>,
}

impl<'a> Cmd<'a> {
    fn new(command: &'a str, args: Option<Vec<&'a str>>) -> Cmd<'a> {
        Cmd { command, args }
    }
}

pub fn build_matches(args: Vec<String>) -> ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("command").index(1).required(true))
        .arg(Arg::with_name("args").min_values(0))
        .get_matches_from(args)
}

pub fn build_cmd(matches: ArgMatches, cmd_list: HashMap<String, String>) -> Result<String> {
    let command = matches
        .value_of("command")
        .context("Command argument not found")?;
    let args = matches.values_of("args").map(|a| a.collect::<Vec<_>>());
    let cmd = Cmd::new(command, args);
    match cmd_list.get(cmd.command) {
        Some(match_cmd) => {
            if let Some(args) = cmd.args {
                Ok(format!("{} {}", match_cmd, args.join(" ")))
            } else {
                Ok(match_cmd.to_string())
            }
        }
        None => Err(anyhow!("Not found command key"))?,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_cmd_test_with_argument() {
        let cmd_list: HashMap<String, String> = vec![
            ("foo".to_string(), "ls".to_string()),
            ("bar".to_string(), "type".to_string()),
        ]
        .into_iter()
        .collect();

        let matches = build_matches(vec![
            "cai".to_string(),
            "bar".to_string(),
            "cat".to_string(),
        ]);
        assert_eq!(
            build_cmd(matches, cmd_list).unwrap(),
            "type cat".to_string()
        )
    }

    #[test]
    fn build_cmd_test_without_argument() {
        let cmd_list: HashMap<String, String> = vec![
            ("foo".to_string(), "ls".to_string()),
            ("bar".to_string(), "type".to_string()),
        ]
        .into_iter()
        .collect();

        let matches = build_matches(vec!["cai".to_string(), "bar".to_string()]);
        assert_eq!(build_cmd(matches, cmd_list).unwrap(), "type".to_string())
    }
}
