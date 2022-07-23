use std::{fmt, fs};
use std::fmt::Formatter;
use std::path::Path;

use ansi_term::Color::Green;
use serde::{Deserialize, Serialize};

#[cfg(test)]
pub mod tests;


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
        write!(f, "{} --- {}",
               Green.bold().paint(&self.name),
               self.url)
    }
}

impl Config {
    pub fn from_path(path: &Path) -> Config {
        let json = fs::read_to_string(path).unwrap_or_else(|err| {
            panic!("could not load config from '{}': {}", path.display(), err)
        });

        serde_json::from_str(&json)
            .unwrap_or_else(|err| panic!("could not deserialize json \n'{}'\n: {}", json, err))
    }

    pub fn location_by_name(&self, name: &str) -> anyhow::Result<&Location> {
        self.locations.iter()
            .find(|loc| loc.name == name)
            .ok_or_else(|| format!("name '{}' not found in the config's locations", name))
            .map_err(anyhow::Error::msg)
    }
}
