use std::{path::PathBuf, io::Error, fmt::Display};

use serde::ser::SerializeStruct;

use crate::files;

pub enum ColorTheme{
    Bright,
    Dark
}

impl serde::ser::Serialize for ColorTheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match &self {
            ColorTheme::Bright => serializer.serialize_unit_variant("ColorTheme", 0, "Bright"),
            ColorTheme::Dark => serializer.serialize_unit_variant("ColorTheme", 1, "Dark"),
        }
    }
}

pub struct DataSet {
    name: String,
    directory: PathBuf
}

impl DataSet {
    pub fn new(name: &str, directory: &str) -> Option<DataSet> {
        let path = PathBuf::from(directory);
        if path.is_dir() {
            Some(DataSet { name: name.to_string(), directory: path })
        } else {
            None
        }
    }
}

impl serde::ser::Serialize for DataSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut state = serializer.serialize_struct("DataSet", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("directory", &self.directory)?;
        state.end()
    }
}

pub struct Config {
    datasets: Vec<DataSet>,
    theme: ColorTheme
}

impl Config {
    pub fn new(theme: ColorTheme) -> Config {
        Config { datasets: Vec::new(), theme}
    }

    pub fn save(&self, name: &str) {
        let serialise = serde_json::to_string(&self).unwrap();
        files::write_file_truncate(name, serialise.as_str());
    }

    pub fn load(name: &str) -> Result<Config, Error> {
        let content = files::read_file(name)?;
        println!("{}", content);
        todo!();
    }
}

impl serde::ser::Serialize for Config {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut state = serializer.serialize_struct("Config", 2)?;
        state.serialize_field("datasets", &self.datasets)?;
        state.serialize_field("theme", &self.theme)?;
        state.end()
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or("Error while reading config".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_write_a_simple_config() {
        let test_file = "config.json";
        let conf = Config::new(ColorTheme::Bright);
        conf.save(test_file);
        println!("{}", &conf);

        let path = Path::new(test_file);
        assert!(path.exists());

        if path.exists() {
            std::fs::remove_file(path).expect("Unable to remove the file");
        }
    }
}
