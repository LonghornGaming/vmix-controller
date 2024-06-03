use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Enable debug output
    #[arg(short, long)]
    debug: bool,

    /// Specify a non-local vMix endpoint
    #[arg(short, long)]
    api: Option<String>,

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
        
        /// Enables streaming to Twitch
        #[arg(short, long)]
        twitch: Option<bool>,

        /// Enables streaming to YouTube
        #[arg(short, long)]
        youtube: Option<bool>,
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

fn main() -> Result<(), reqwest::Error> {
    let cli = Cli::parse();
    let client = reqwest::blocking::Client::new();

    let url = match &cli.api {
        None => "http://127.0.0.1:8088",
        Some(api) => api,
    }
    .to_owned()
        + "/api";

    // if cli.debug {
    //     let state = client.get(&url).send()?.text()?;
    //     println!("{}", &state);
    // }

    match &cli.command {
        Commands::Start { input, twitch, youtube } => {
            // Switch to start
            client
                .get(&url)
                .query(&[
                    ("Function", "CutDirect"),
                    ("Input", &input.clone().unwrap_or(1.to_string())),
                ])
                .send()?;
            
            
            // Start streaming
            if cli.debug {
                client 
                    .get(&url)
                    .query(&[
                        ("Function", "StartStreaming"),
                        ("Value", "3")
                    ])
                    .send()?;

                return Ok(())
            }

            if twitch.unwrap_or(false) {
                client 
                    .get(&url)
                    .query(&[
                        ("Function", "StartStreaming"),
                        ("Value", "0")
                    ])
                    .send()?;
            }

            if youtube.unwrap_or(false) {
                client 
                    .get(&url)
                    .query(&[
                        ("Function", "StartStreaming"),
                        ("Value", "1")
                    ])
                    .send()?;
            }
        }
        Commands::Break { input } => {
            client
                .get(&url)
                // Switch to break
                .query(&[
                    ("Function", "CutDirect"),
                    ("Input", &input.clone().unwrap_or(2.to_string())),
                ])
                .send()?;
        }
        Commands::Game { input } => {
            client
                .get(&url)
                // Switch to game
                .query(&[
                    ("Function", "CutDirect"),
                    ("Input", &input.clone().unwrap_or(3.to_string())),
                ])
                .send()?;
        }
        Commands::End { input } => {
            client
                .get(&url)
                // Switch to end
                .query(&[
                    ("Function", "CutDirect"),
                    ("Input", &input.clone().unwrap_or(4.to_string())),
                ])
                .send()?;
        }
    }
    Ok(())
}
