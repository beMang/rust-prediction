use std::{path::PathBuf, io::Error, fmt::Display};
use serde::{Serialize, Deserialize};

use crate::files;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum ColorTheme{
    Bright,
    Dark
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
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

#[derive(Deserialize, Serialize)]
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
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
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

    struct TestContext {
        test_file: String
    }

    impl TestContext {
        fn new() -> TestContext {
            TestContext { test_file: String::from("config_test.json") }
        }

        fn get_basic_config(&self) -> Config {
            let mut config = Config::new(ColorTheme::Bright);
            config.datasets.push(DataSet { name: String::from("test"), directory: PathBuf::from(&self.test_file) });
            config
        }
    }

    impl  Drop for TestContext {
        fn drop(&mut self) {
            let path = Path::new(self.test_file.as_str());

            if path.exists() {
                std::fs::remove_file(path).expect("Unable to remove the file");
            }
        }
    }
    
    #[serial_test::serial] //serial to avoid parallel acess :)
    #[test]
    fn test_write_a_simple_config() {
        let context = TestContext::new();
        let conf = context.get_basic_config();
        conf.save(&context.test_file);
        println!("{}", &conf);

        let path = Path::new(&context.test_file);
        assert!(path.exists());
    }

    #[serial_test::serial]
    #[test]
    fn test_read_a_simple_config() {
        let context = TestContext::new();
        let conf = context.get_basic_config();
        conf.save(&context.test_file);
        
        let save_conf = Config::load(&context.test_file).unwrap();

        assert_eq!(conf.datasets, save_conf.datasets);
        assert_eq!(conf.theme, save_conf.theme);
    }
}
