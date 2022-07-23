use std::path::PathBuf;

use clap::clap_derive::Parser;
use lazy_static::lazy_static;

use command::Command;

pub mod command;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[clap(short, long, value_parser, value_name = "FILE", default_value = & DEFAULT_CONFIG_PATH)]
    pub config: PathBuf,

    /// Control verbosity
    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    #[clap(subcommand)]
    pub command: Command,
}

lazy_static! {
    static ref DEFAULT_CONFIG_PATH: String = default_config();
}

const DEFAULT_CONFIG_NAME: &str = ".mover.json";

fn default_config() -> String {
    let mut home = home::home_dir().expect("could not find home dir for default config");
    home.push(DEFAULT_CONFIG_NAME);
    home.as_path().display().to_string()
}
