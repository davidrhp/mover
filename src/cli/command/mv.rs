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
    fn execute(&self, cfg: Config) -> anyhow::Result<()>{
        println!("config: {:#?}", cfg);
        Ok(())
    }
}
