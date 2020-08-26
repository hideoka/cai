use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn parse_config_file<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let cmd_list = serde_json::from_str::<HashMap<String, String>>(&content)?;
    Ok(cmd_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config_file_test() {
        let result: HashMap<String, String> = vec![
            ("foo".to_string(), "ls".to_string()),
            ("bar".to_string(), "type".to_string()),
        ]
        .into_iter()
        .collect();
        assert_eq!(
            parse_config_file("./example/cai_config.json").unwrap(),
            result
        )
    }
}
