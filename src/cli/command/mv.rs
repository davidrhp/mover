use std::error::Error;

use clap::clap_derive::Args;

use crate::cli::command::Execute;
use crate::config::Config;

#[derive(Args)]
pub struct Move {
    /// Source location, whose content will be copied
    #[clap(value_name = "SRC")]
    src: String,

    /// Destination location, whose content will be overridden
    #[clap(value_name = "DST")]
    dst: String,
}

impl Execute for Move {
    fn execute(&self, cfg: Config) -> Result<(), Box<dyn Error>> {
        println!("config: {:#?}", cfg);
        Ok(())
    }
}
