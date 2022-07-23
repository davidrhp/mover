use std::io;

use anyhow::Context;
use clap::clap_derive::Args;

use crate::cli::command::Execute;
use crate::config::Config;

#[cfg(test)]
mod tests;

#[derive(Args)]
pub struct List {}

impl Execute for List {
    fn execute(&self, cfg: Config) -> anyhow::Result<()> {
        list_action(&mut io::stdout().lock(), &cfg)
    }
}

fn list_action(mut w: impl io::Write, cfg: &Config) -> anyhow::Result<()> {
    cfg.locations.iter()
        .try_for_each(|loc| writeln!(w, "{}", loc))
        .with_context(|| "failed to write to writer")
}
