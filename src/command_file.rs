use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub struct CommandFileService {
    kind: CommandFileKind,
    path: Option<String>,
}

impl CommandFileService {
    pub fn new(config_file_kind: Option<String>, config_file_path: Option<String>) -> Self {
        let kind = match config_file_kind.as_ref().map(AsRef::as_ref) {
            Some("JSON") => CommandFileKind::Json,
            Some("YAML") => CommandFileKind::Yaml,
            _ => CommandFileKind::Json,
        };
        let path = config_file_path;
        CommandFileService { kind, path }
    }

    pub fn parse_config_file(&self) -> Result<HashMap<String, String>> {
        let path = self.path.as_ref().map(AsRef::as_ref);
        let command_file: Box<dyn CommandFile> = match self.kind {
            CommandFileKind::Json => Box::new(JsonCommandFile::new(path)),
            CommandFileKind::Yaml => Box::new(YamlCommandFile::new(path)),
        };

        let mut file = File::open(command_file.path())?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let cmd_list = command_file.parse(content)?;
        Ok(cmd_list)
    }
}

enum CommandFileKind {
    Json,
    Yaml,
}

struct JsonCommandFile<'a> {
    path: &'a str,
}

struct YamlCommandFile<'a> {
    path: &'a str,
}

trait CommandFile {
    fn parse(&self, content: String) -> Result<HashMap<String, String>>;
    fn path(&self) -> &str;
}

impl<'a> JsonCommandFile<'a> {
    fn new(path: Option<&'a str>) -> Self {
        let file_path = path.unwrap_or("./cai_config.json");
        JsonCommandFile { path: file_path }
    }
}

impl<'a> CommandFile for JsonCommandFile<'a> {
    fn parse(&self, content: String) -> Result<HashMap<String, String>> {
        let cmd_list = serde_yaml::from_str::<HashMap<String, String>>(&content)?;
        Ok(cmd_list)
    }
    fn path(&self) -> &str {
        &self.path
    }
}

impl<'a> YamlCommandFile<'a> {
    fn new(path: Option<&'a str>) -> Self {
        let file_path = path.unwrap_or("./cai_config.yaml");
        YamlCommandFile { path: file_path }
    }
}

impl<'a> CommandFile for YamlCommandFile<'a> {
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

        let command_file_service =
            CommandFileService::new(None, Some("./example/cai_config.json".to_string()));
        assert_eq!(command_file_service.parse_config_file().unwrap(), result)
    }

    #[test]
    fn parse_yaml_config_file_test() {
        let result: HashMap<String, String> = vec![
            ("foo".to_string(), "ls".to_string()),
            ("bar".to_string(), "type".to_string()),
        ]
        .into_iter()
        .collect();

        let command_file_service =
            CommandFileService::new(None, Some("./example/cai_config.yaml".to_string()));
        assert_eq!(command_file_service.parse_config_file().unwrap(), result)
    }
}
