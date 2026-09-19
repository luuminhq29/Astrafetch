mod cli;
mod config;
mod output;
mod system;
mod ui;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use config::Config;
use output::{json, minimal};
use system::SystemSnapshot;

const VERSION: &str = "1.0.0";
const AUTHOR: &str = "Lưu Minh Quang - Astra";

fn main() -> Result<()> {
    let cli = Cli::parse();
    if cli.version_requested {
        println!("AstraFetch {VERSION}");
        println!("Author: {AUTHOR}");
        return Ok(());
    }

    let mut config = Config::load()?;
    config.apply_cli(&cli);

    if cli.json {
        let snapshot = SystemSnapshot::collect(&config)?;
        json::print_json(&snapshot)?;
    } else if cli.minimal {
        let snapshot = SystemSnapshot::collect(&config)?;
        println!("{}", minimal::format(&snapshot));
    } else if cli.watch {
        ui::watch::run(config)?;
    } else {
        ui::static_view::run(config)?;
    }
    Ok(())
}
