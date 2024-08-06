use vmix;
use youtube;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use log::info;
use serde::{Deserialize, Serialize};

use mimalloc::MiMalloc;

use std::fs::File;
use std::io::Write;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

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
    /// Run alerts
    Alerts {
        #[command(subcommand)]
        input: AlertCommands,
    },
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
const VMIX_ENDPOINT: &str = "127.0.0.1:8088";

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Dump { input } => match input {
            &DumpCommands::VmixInputs => {
                let mut vmix = vmix::client::Client::new(VMIX_ENDPOINT)?;
                println!("\nInputs: {:#?}", vmix.inputs());
            }

            &DumpCommands::VmixTitles => {
                let mut vmix = vmix::client::Client::new(VMIX_ENDPOINT)?;
                println!("\nTitles: {:#?}", vmix.titles());
            }

            &DumpCommands::VmixXml => {
                let mut vmix = vmix::client::Client::new(VMIX_ENDPOINT)?;
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

        Commands::Alerts { input } => match input {
            &AlertCommands::Twitch => {
                todo!()
            }
            &AlertCommands::Youtube => {
                todo!()
            }
        },
    }
    Ok(())
}
