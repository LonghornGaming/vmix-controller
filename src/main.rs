use vmix;
mod config;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use log::info;
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::Write;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Enable debug mode (streaming won't be started)
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Serialize, Deserialize, Debug)]
enum Commands {
    /// Dump given value
    Dump {
        #[command(subcommand)]
        input: DumpCommands,
    },
    /// Switch to a given input
    Switch {
        #[command(subcommand)]
        input: SwitchCommands,
    },
    /// Increment a counter in a title
    Inc {
        /// The title
        #[arg(short, long)]
        title: vmix::Input,
        /// The index of the text to increment
        #[arg(short, long)]
        idx: Option<u32>,
    },
    /// Run alerts
    Alerts {
        #[command(subcommand)]
        input: AlertCommands,
    }
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

#[derive(
    Subcommand, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
enum DumpCommands {
    /// Dump the raw XML state from vMix
    Xml,
    /// Get the available inputs
    Inputs,
    /// Get the available titles
    Titles,
}

#[derive(
    Subcommand, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
enum AlertCommands {
    /// YouTube Alerts
    Youtube,
    /// Twitch Alerts
    Twitch,
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

    let mut vmix = vmix::client::Client::new(&cfg.endpoint)?;

    match &cli.command {
        Commands::Dump { input } => match input {
            &DumpCommands::Inputs => {
                println!("\nInputs: {:#?}", vmix.inputs());
            }

            &DumpCommands::Titles => {
                println!("\nTitles: {:#?}", vmix.titles());
            }

            &DumpCommands::Xml => {
                File::create("last_state.xml")?.write_all(vmix.xml()?.as_bytes())?;

                println!("Parsed State: {:#?}", vmix.state()?);
            }
        },

        Commands::Inc { title, idx } => {
            let text = vmix.get_text(title, *idx)?;
            let val: u32 = text.parse()?;
            vmix.set_text(title, *idx, (val + 1).to_string())?;
        }

        Commands::Switch { input } => {
            match input {
                &SwitchCommands::Start => {
                    // Switch to start
                    vmix.quick_play(cfg.inputs.at(*input)?)?;

                    // Start streaming
                    info!("Starting streaming");
                    if cli.debug {
                        return Ok(()); // Don't stream in debug mode
                    }

                    vmix.start_streaming()?;
                }
                _ => {
                    vmix.quick_play(cfg.inputs.at(*input)?)?;
                }
            }
        }

        Commands::Alerts { input } => {
            todo!()
        }
    }
    Ok(())
}
