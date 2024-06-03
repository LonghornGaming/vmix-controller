use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Enable debug output
    #[arg(short, long)]
    debug: bool,

    /// vMix API endpoint (http://127.0.0.1:8088 by default)
    #[arg(short, long)]
    api: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Switch to the starting input, and begin streaming
    Start {
        /// The input
        #[arg(short, long)]
        input: Option<String>,
    },
    /// Switch to the break input
    Break {
        /// The input
        #[arg(short, long)]
        input: Option<String>,
    },
    /// Switch to the game input
    Game {
        /// The input
        #[arg(short, long)]
        input: Option<String>,
    },
    /// Switch to the ending input, and end streaming
    End {
        /// The input
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

    // let state = api.try_clone().unwrap().send()?.text()?;
    let state = client.get(&url).send()?.text()?;
    if cli.debug {
        println!("{}", &state);
    }

    match &cli.command {
        Commands::Start { input } => {
            // Switch to start
            client
                .get(&url)
                .query(&[
                    ("Function", "ActiveInput"),
                    ("Input", &input.clone().unwrap_or(1.to_string())),
                ])
                .send()?;

        }
        Commands::Break { input } => {
            client
                .get(&url)
                // Switch to break
                .query(&[
                    ("Function", "ActiveInput"),
                    ("Input", &input.clone().unwrap_or(2.to_string())),
                ])
                .send()?;
        }
        Commands::Game { input } => {
            client
                .get(&url)
                // Switch to game
                .query(&[
                    ("Function", "ActiveInput"),
                    ("Input", &input.clone().unwrap_or(3.to_string())),
                ])
                .send()?;
        }
        Commands::End { input } => {
            client
                .get(&url)
                // Switch to end
                .query(&[
                    ("Function", "ActiveInput"),
                    ("Input", &input.clone().unwrap_or(4.to_string())),
                ])
                .send()?;
        }
    }

    Ok(())
}
