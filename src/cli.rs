use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, about, long_about = None)]
#[command(version, propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Send data to meili server
    Send,
    /// Search with the given query
    Search { query: String },
}

/// Parse and run command.
pub async fn app() -> crate::Result<()> {
    use crate::{search, send_data};

    let args = Args::parse();
    match args.command {
        Command::Send => send_data().await?,
        Command::Search { query } => search(&query).await?,
    };
    Ok(())
}
