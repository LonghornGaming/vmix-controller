mod client;
mod config;
mod xml;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use log::info;
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

#[derive(Subcommand, Serialize, Deserialize, Debug)]
enum Commands {
    /// Get the available inputs
    Inputs,
    /// Get the available titles
    Titles,
    /// Switch to a given input
    Switch {
        #[command(subcommand)]
        input: SwitchCommands,
    },
    /// Increment a counter in a title
    Inc {
        /// The title
        #[arg(short, long)]
        input: crate::config::Input,
        /// The index of the text to increment
        #[arg(short, long)]
        title: u32,
    },
}

#[derive(
    Subcommand, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
enum SwitchCommands {
    /// Switch to the starting input, and begin streaming
    Start,
    /// Switch to the break input
    Break,
    /// Switch to the game input
    Game,
    /// Switch to the ending input
    End,
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    info!(
        "Config: {:?}",
        confy::get_configuration_file_path("vmix-controller", None)
            .with_context(|| "Bad configuration file")?
    );
    let cfg: config::Config = confy::load("vmix-controller", None)?;

    let vmix = client::Client::new(&cfg, cli.dump_xml)?;

    match &cli.command {
        Commands::Inputs => {
            let inputs = vmix.inputs();
    
            println!("\nInputs: {:#?}", inputs);
        }

        Commands::Titles => {
            use crate::xml::Input;
            let inputs = vmix.inputs();
    
            println!("\nTitles: {:#?}", inputs.iter().filter(|i: &&Input| i.kind == "GT").collect::<Vec<&Input>>());
        }

        Commands::Inc { input, title } => {

        }

        Commands::Switch { input } => {
            match input {
                &SwitchCommands::Start => {
                    // Switch to start
                    vmix.quick_play(cfg.inputs.at(*input)?)?;

                    // Start streaming
                    info!("Starting streaming");
                    if cli.debug {
                        return Ok(());
                    }

                    vmix.start_streaming()?;
                }
                _ => {
                    vmix.quick_play(cfg.inputs.at(*input)?)?;
                }
            }
        }
    }
    Ok(())
}
