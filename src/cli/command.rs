use clap::clap_derive::Subcommand;

use crate::cli::command::ls::List;
use crate::cli::command::mv::Move;
use crate::config::Config;

mod ls;
mod mv;

#[derive(Subcommand)]
pub enum Command {
    /// List available locations
    #[clap(alias("ls"))]
    List(List),
    /// Move <SRC> to <DST>
    #[clap(alias("mv"))]
    Move(Move),
}

impl Execute for Command {
    fn execute(&self, cfg: Config) -> anyhow::Result<()> {
        match self {
            Self::List(cmd) => cmd.execute(cfg),
            Self::Move(cmd) => cmd.execute(cfg),
        }
    }
}

pub trait Execute {
    fn execute(&self, cfg: Config) -> anyhow::Result<()>;
}
