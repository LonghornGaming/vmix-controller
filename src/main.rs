mod client;
mod config;
mod xml;

use clap::{Parser, Subcommand};
use log::info;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Enable debug mode (streaming won't be started)
    #[arg(short, long)]
    debug: bool,

    /// Dump vMix XML to file
    #[arg(long)]
    dump_xml: bool,

    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Commands {
    /// Switch to the starting input, and begin streaming
    Start,
    /// Switch to the break input
    Break,
    /// Switch to the game input
    Game,
    /// Switch to the ending input, and end streaming
    End,
    /// Get the available inputs
    Inputs,
}


fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    info!("Config: {:?}", confy::get_configuration_file_path("vmix-controller", None).with_context(|| "Bad configuration file")?);
    let cfg: config::Config = confy::load("vmix-controller", None)?;

    let vmix = client::Client::new(&cfg, cli.dump_xml)?;
    
    match &cli.command {
        Commands::Start => {
            // Switch to start
            vmix.quick_play(cfg.inputs.at(cli.command)?)?;

            // Start streaming
            info!("Starting streaming");
            if cli.debug {
                return Ok(())
            }

            vmix.start_streaming()?;
        }
        Commands::Break => {
            vmix.quick_play(cfg.inputs.at(cli.command)?)?;
        }
        Commands::Game => {
            vmix.quick_play(cfg.inputs.at(cli.command)?)?;
        }
        Commands::End  => {
            vmix.quick_play(cfg.inputs.at(cli.command)?)?;
        }
        Commands::Inputs => {
            println!("\n{:#?}", vmix.inputs())
        }
    }
    Ok(())
}