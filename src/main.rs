mod client;
mod config;
mod xml;

use clap::{Parser, Subcommand};
use log::info;
use anyhow::{Context, Result};

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


#[derive(Subcommand)]
enum Commands {
    /// Switch to the starting input, and begin streaming
    Start {
        /// The input to use
        #[arg(short, long)]
        input: Option<String>,
    },
    /// Switch to the break input
    Break {
        /// The input to use
        #[arg(short, long)]
        input: Option<String>,
    },
    /// Switch to the game input
    Game {
        /// The input to use
        #[arg(short, long)]
        input: Option<String>,
    },
    /// Switch to the ending input, and end streaming
    End {
        /// The input to use
        #[arg(short, long)]
        input: Option<String>,
    },
}



fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    info!("Config: {:?}", confy::get_configuration_file_path("vmix-controller", None).with_context(|| "Bad configuration file")?);
    let cfg: config::Config = confy::load("vmix-controller", None)?;

    let vmix = client::Client::new(cfg, cli.dump_xml)?;

    match &cli.command {
        Commands::Start { input } => {
            // Switch to start
            vmix.quick_play(&input.clone().unwrap_or(1.to_string()))?;

            // Start streaming
            info!("Starting streaming");
            if cli.debug {
                return Ok(())
            }

            vmix.start_streaming()?;
        }
        Commands::Break { input } => {
            vmix.quick_play(&input.clone().unwrap_or(2.to_string()))?;
        }
        Commands::Game { input } => {
            vmix.quick_play(&input.clone().unwrap_or(3.to_string()))?;
        }
        Commands::End { input } => {
            vmix.quick_play(&input.clone().unwrap_or(4.to_string()))?;
        }
    }
    Ok(())
}