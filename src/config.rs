use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    config_file: String,
    source: String,
    destination: String,
    log_file: String,
}

impl Config {
    pub fn new(config_file: &str, source: &str, destination: &str, log_file: &str) -> Self {
        let mut file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_file)
            .expect("Cannot create the config file!");
        let config = Self {
            config_file: config_file.to_string(),
            source: source.to_string(),
            destination: destination.to_string(),
            log_file: log_file.to_string(),
        };
        // I did not find how to write a json object on multiple lines
        // so I did it manually
        file.write(
            serde_json::to_string(&config)
                .expect("Cannot parse the config struct!")
                .replace("{", "{\n\t")
                .replace(",", ",\n\t")
                .replace("}", "\n}")
                .as_bytes(),
        )
        .expect("Cannot write the config file!");
        config
    }

    pub fn from(config_file: &str) -> Self {
        let mut file = File::open(config_file).expect("Cannot open the config file!");
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .expect("Cannot read the config file!");
        serde_json::from_str(&file_content).expect("Cannot parse the config file!")
    }

    pub fn initialize() -> Self {
        let config_dir = env::current_exe()
            .unwrap()
            .to_path_buf()
            .parent()
            .expect("Cannot find parent directory of the application!")
            .join("config");

        if !config_dir.exists() {
            fs::create_dir_all(config_dir.clone()).expect("Cannot create config directory!");

            Config::new(
                config_dir.join("config.json").to_str().unwrap(),
                "source/",
                "destination/",
                config_dir.join("log.txt").to_str().unwrap(),
            )
        } else {
            Config::from(config_dir.join("config.json").to_str().unwrap())
        }
    }

    pub fn config_file(&self) -> &str {
        &self.config_file
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn destination(&self) -> &str {
        &self.destination
    }

    pub fn log_file(&self) -> &str {
        &self.log_file
    }

    pub fn source_as_path(&self) -> &Path {
        Path::new(&self.source)
    }
    pub fn destination_as_path(&self) -> &Path {
        Path::new(&self.destination)
    }
}
