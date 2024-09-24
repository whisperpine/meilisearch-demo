use anyhow::Result;
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

#[tokio::main]
async fn main() -> Result<()> {
    use meilisearch_demo::{search, send_data};

    init_tracing();

    let args = Args::parse();
    match args.command {
        Command::Send => send_data().await?,
        Command::Search { query } => search(&query).await?,
    };

    Ok(())
}

fn init_tracing() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
