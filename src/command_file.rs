use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub struct CommandFileConfig {
    kind: CommandFileKind,
    path: Option<String>,
}

impl CommandFileConfig {
    pub fn new(config_file_kind: Option<String>, config_file_path: Option<String>) -> Self {
        let kind = match config_file_kind.as_ref().map(AsRef::as_ref) {
            Some("JSON") => CommandFileKind::Json,
            Some("YAML") => CommandFileKind::Yaml,
            _ => CommandFileKind::Json,
        };
        let path = config_file_path;
        CommandFileConfig { kind, path }
    }
}

pub fn parse_config_file(config: CommandFileConfig) -> Result<HashMap<String, String>> {
    let command_file: Box<dyn CommandFile> = match config.kind {
        CommandFileKind::Json => Box::new(JsonCommandFile::new(config.path)),
        CommandFileKind::Yaml => Box::new(YamlCommandFile::new(config.path)),
    };

    let mut file = File::open(command_file.path())?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let cmd_list = command_file.parse(content)?;
    Ok(cmd_list)
}

enum CommandFileKind {
    Json,
    Yaml,
}

struct JsonCommandFile {
    path: String,
}

struct YamlCommandFile {
    path: String,
}

trait CommandFile {
    fn parse(&self, content: String) -> Result<HashMap<String, String>>;
    fn path(&self) -> &str;
}

impl JsonCommandFile {
    fn new(path: Option<String>) -> Self {
        let file_path = path.unwrap_or("./cai_config.json".to_string());
        JsonCommandFile { path: file_path }
    }
}

impl CommandFile for JsonCommandFile {
    fn parse(&self, content: String) -> Result<HashMap<String, String>> {
        let cmd_list = serde_yaml::from_str::<HashMap<String, String>>(&content)?;
        Ok(cmd_list)
    }
    fn path(&self) -> &str {
        &self.path
    }
}

impl YamlCommandFile {
    fn new(path: Option<String>) -> Self {
        let file_path = path.unwrap_or("./cai_config.yaml".to_string());
        YamlCommandFile { path: file_path }
    }
}

impl CommandFile for YamlCommandFile {
    fn parse(&self, content: String) -> Result<HashMap<String, String>> {
        let cmd_list = serde_yaml::from_str::<HashMap<String, String>>(&content)?;
        Ok(cmd_list)
    }
    fn path(&self) -> &str {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_json_config_file_test() {
        let result: HashMap<String, String> = vec![
            ("foo".to_string(), "ls".to_string()),
            ("bar".to_string(), "type".to_string()),
        ]
        .into_iter()
        .collect();

        let config = CommandFileConfig::new(None, Some("./example/cai_config.json".to_string()));
        assert_eq!(parse_config_file(config).unwrap(), result)
    }

    #[test]
    fn parse_yaml_config_file_test() {
        let result: HashMap<String, String> = vec![
            ("foo".to_string(), "ls".to_string()),
            ("bar".to_string(), "type".to_string()),
        ]
        .into_iter()
        .collect();

        let config = CommandFileConfig::new(None, Some("./example/cai_config.yaml".to_string()));
        assert_eq!(parse_config_file(config).unwrap(), result)
    }
}
