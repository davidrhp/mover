use std::fs;
use std::io::stdout;
use std::io::Write;

use ansi_term::Color::Red;
use ansi_term::Style;
use anyhow::Context;
use clap::clap_derive::Args;

use crate::cli::command::Execute;
use crate::config::{Config, Location};

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
    fn execute(&self, cfg: Config) -> anyhow::Result<()> {
        let src_loc = cfg.location_by_name(&self.src)?;
        let dst_loc = cfg.location_by_name(&self.dst)?;
        mv_local(stdout().lock(), src_loc, dst_loc)
    }
}

fn mv_local(mut w: impl Write, from: &Location, to: &Location) -> anyhow::Result<()> {
    fs::rename(from.url.path(), to.url.path())
        .with_context(|| mv_msg_failure(from, to))?;

    writeln!(w, "{}", mv_msg_success(from, to))
        .map_err(anyhow::Error::new)
}

fn mv_msg_details(from: &Location, to: &Location) -> String {
    format!("{} -> {}", from, to)
}

fn mv_msg_success(from: &Location, to: &Location) -> String {
    format!("{} {}", Style::new().bold().paint("moved"), mv_msg_details(from, to))
}

fn mv_msg_failure(from: &Location, to: &Location) -> String {
    format!("{} {}", Red.bold().paint("could not move"), mv_msg_details(from, to))
}
