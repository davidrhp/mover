use std::{fmt, fs};
use std::fmt::Formatter;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub locations: Vec<Location>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub url: url::Url,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.name, self.url)
    }
}

impl Config {
    pub fn from_path(path: &PathBuf) -> Config {
        let json = fs::read_to_string(path).unwrap_or_else(|err| {
            panic!("could not load config from '{}': {}", path.display(), err)
        });

        serde_json::from_str(&json)
            .unwrap_or_else(|err| panic!("could not deserialize json \n'{}'\n: {}", json, err))
    }
}
