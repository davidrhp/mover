use std::error::Error;

use clap::Parser;

use mover::cli::Cli;
use mover::cli::command::Execute;
use mover::config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = Cli::parse();
    init_logger(&cli);

    println!("Value for config path: {}", cli.config.display());

    let cfg = Config::from_path(&cli.config);
    cli.command.execute(cfg)
}

fn init_logger(cli: &Cli) {
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    log::info!("Log level: {}", cli.verbose.log_level_filter());
}
