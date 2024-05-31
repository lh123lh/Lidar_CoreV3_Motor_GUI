use anyhow::{bail, Context, Result};
use serde::de::DeserializeOwned;
use serde_yaml;

pub fn parse_str<T: std::str::FromStr>(target: &str, key: &str) -> Option<T> {
    let parts: Vec<&str> = target.split(";").collect();
    for part in parts {
        let sub: Vec<&str> = part.splitn(2, "=").collect();
        if sub[0].trim() == key {
            return sub[1].parse::<T>().ok();
        }
    }
    None
}

pub fn is_file_exist(file: &str) -> bool {
    if std::fs::metadata(file).is_ok() {
        true
    } else {
        false
    }
}

pub fn save_yaml(yaml: String, file: &str) -> std::io::Result<()> {
    std::fs::write(file, yaml)?;
    Ok(())
}

pub fn read_yaml<T: DeserializeOwned>(file: &str) -> Result<T> {
    if is_file_exist(file) {
        let yaml_str = std::fs::read_to_string(file)
            .with_context(|| format!("failed to read the file \"{}\"", file))?;

        serde_yaml::from_str::<T>(&yaml_str)
            .with_context(|| format!("failed to read the file with yaml format \"{}\"", file))
    } else {
        bail!("File not exist")
    }
}