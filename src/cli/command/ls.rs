use std::error::Error;

use clap::clap_derive::Args;

use crate::cli::command::Execute;
use crate::config::Config;

#[derive(Args)]
pub struct List {}

impl Execute for List {
    fn execute(&self, cfg: Config) -> Result<(), Box<dyn Error>> {
        println!("config: {:#?}", cfg);

        Ok(())
    }
}
