use anyhow::{anyhow, Context, Result};
use clap::{crate_authors, crate_name, crate_version, App, AppSettings, Arg, ArgMatches};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub struct Cmd<'a> {
    command: &'a str,
    args: Option<Vec<&'a str>>,
}

impl<'a> Cmd<'a> {
    pub fn new(matches: &'a ArgMatches) -> Result<Cmd<'a>> {
        let command = matches
            .value_of("command")
            .context("Command argument not found")?;
        let args = matches.values_of("args").map(|a| a.collect::<Vec<_>>());
        Ok(Cmd { command, args })
    }

    pub fn build_cmd(&self, cmd_list: HashMap<String, String>) -> Result<String> {
        match cmd_list.get(self.command) {
            Some(match_cmd) => {
                if let Some(args) = &self.args {
                    Ok(format!("{} {}", match_cmd, args.join(" ")))
                } else {
                    Ok(match_cmd.to_string())
                }
            }
            None => Err(anyhow!("Not found command key"))?,
        }
    }
}

pub fn build_config_file() -> Result<()> {
    let mut file = File::create("cai_config.json")?;
    let content = "{\n\n}";
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn build_matches(args: Vec<String>) -> ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::AllowLeadingHyphen)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("command").index(1).required(true))
        .arg(Arg::with_name("args").multiple(true))
        .get_matches_from(args)
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
            Cmd::new(&matches).unwrap().build_cmd(cmd_list).unwrap(),
            "type cat".to_string()
        )
    }

    #[test]
    fn build_cmd_test_with_two_argument() {
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
            "echo".to_string(),
        ]);

        assert_eq!(
            Cmd::new(&matches).unwrap().build_cmd(cmd_list).unwrap(),
            "type cat echo".to_string()
        )
    }

    #[test]
    fn build_cmd_test_with_one_argument_and_hyphen_argument() {
        let cmd_list: HashMap<String, String> = vec![
            ("foo".to_string(), "ls".to_string()),
            ("bar".to_string(), "type".to_string()),
        ]
        .into_iter()
        .collect();

        let matches = build_matches(vec![
            "cai".to_string(),
            "foo".to_string(),
            "-alt".to_string(),
            "./".to_string(),
        ]);

        assert_eq!(
            Cmd::new(&matches).unwrap().build_cmd(cmd_list).unwrap(),
            "ls -alt ./".to_string()
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
        assert_eq!(
            Cmd::new(&matches).unwrap().build_cmd(cmd_list).unwrap(),
            "type".to_string()
        )
    }
}
