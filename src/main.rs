use vmix;
use youtube;
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

#[derive(Subcommand, Serialize, Deserialize, Debug)]
enum DumpCommands {
    /// Dump the raw XML state from vMix
    VmixXml,
    /// Get the available inputs
    VmixInputs,
    /// Get the available titles
    VmixTitles,
    /// YT Broadcasts
    YTBroadcasts {
        /// The api key
        #[arg(short, long)]
        key: String,
    },
    /// YT Live Chat
    YTChat {
        /// The api key
        #[arg(short, long)]
        key: String,
        /// The chat id
        #[arg(short, long)]
        id: String,
    },
}

#[derive(Subcommand, Serialize, Deserialize, Debug)]
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

    // let mut vmix = vmix::client::Client::new(&cfg.endpoint)?;

    match &cli.command {
        Commands::Dump { input } => match input {
            &DumpCommands::VmixInputs => {
                let mut vmix = vmix::client::Client::new(&cfg.endpoint)?;
                println!("\nInputs: {:#?}", vmix.inputs());
            }

            &DumpCommands::VmixTitles => {
                let mut vmix = vmix::client::Client::new(&cfg.endpoint)?;
                println!("\nTitles: {:#?}", vmix.titles());
            }

            &DumpCommands::VmixXml => {
                let mut vmix = vmix::client::Client::new(&cfg.endpoint)?;
                File::create("last_state.xml")?.write_all(vmix.xml()?.as_bytes())?;

                println!("Parsed State: {:#?}", vmix.state()?);
            }

            DumpCommands::YTBroadcasts { key } => {
                let yt = youtube::client::Client::new(key.to_string())?;

                println!("Broadcasts: {}", yt.get_broadcasts(None)?);
            }

            DumpCommands::YTChat { key, id } => {
                let yt = youtube::client::Client::new(key.to_string())?;

                println!("Parsed Chat: {:#?}", yt.get_chat(id, None)?);
            }
        },

        Commands::Inc { title, idx } => {
            let mut vmix = vmix::client::Client::new(&cfg.endpoint)?;
            let text = vmix.get_text(title, *idx)?;
            let val: u32 = text.parse()?;
            vmix.set_text(title, *idx, (val + 1).to_string())?;
        }

        Commands::Switch { input } => {
            let vmix = vmix::client::Client::new(&cfg.endpoint)?;
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
