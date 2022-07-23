use std::path::PathBuf;
use std::str::FromStr;

use assert_fs::prelude::*;

use super::*;

const TEST_CONFIG_VALID: &str = "./testdata/config/config.json";
const EXAMPLE_FILE: &str = "exampleFile";
const EXAMPLE_DIR: &str = "exampleDir";
const EXAMPLE_URL: &str = "file:///Users/davidrhp/go/src/mover/testdata/txt/example.txt";

pub fn test_config() -> Config {
    Config{
        locations: vec![test_location()]
    }
}

pub fn test_location() -> Location {
    Location {
        name: String::from(EXAMPLE_FILE),
        url: url::Url::from_str(EXAMPLE_URL)
            .expect("could not create test location"),
    }
}

#[test]
fn from_path() -> anyhow::Result<()> {
    let path = PathBuf::from_str(TEST_CONFIG_VALID)
        .expect("Could not load test file");

    let cfg = Config::from_path(&path);

    let expected_names = vec![EXAMPLE_FILE, EXAMPLE_DIR];
    expected_names.iter()
        .zip(cfg.locations)
        .for_each(|(expected_name, location)| assert_eq!(*expected_name, location.name));

    Ok(())
}

#[test]
#[should_panic(expected= "could not load config from")]
fn from_path_file_not_found() {
    let dir = assert_fs::TempDir::new().unwrap();
    let file = dir.child("nonExistentFile.json");

    Config::from_path(file.path());
}

#[test]
fn location_display() -> anyhow::Result<()>{
    let loc = test_location();

    let actual = format!("{}", loc);
    assert!(actual.contains(EXAMPLE_FILE));
    assert!(actual.contains(EXAMPLE_URL));

    Ok(())
}
