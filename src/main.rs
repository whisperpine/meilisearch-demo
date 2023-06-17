use anyhow::Result;
use clap::{Parser, Subcommand};
use meilisearch_sdk::client::Client;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

const DATA_FILE_PATH: &str = "movies.json";
const INDEX_NAME: &str = "movies";

static MEILI_CLIENT: OnceLock<Client> = OnceLock::new();

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    id: i64,
    title: String,
    poster: String,
    overview: String,
    release_date: i64,
    genres: Vec<String>,
}

impl std::fmt::Display for Movie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(id: {}, title: {})", self.id, self.title)
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Send data to meili server
    Send,
    ///  Search with the given query
    Search { query: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing().await;
    init_meili_client();

    let args = Args::parse();

    match args.command {
        Command::Send => send_data().await?,
        Command::Search { query } => search(&query)
            .await
            .unwrap_or_else(|err| tracing::error!(?err)),
    };

    Ok(())
}

fn init_meili_client() {
    MEILI_CLIENT.get_or_init(|| Client::new("http://localhost:7700", Some("master-key")));
}

async fn init_tracing() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Read json file from local dir, and send data to meili server.
pub async fn send_data() -> Result<()> {
    let content = tokio::fs::read_to_string(DATA_FILE_PATH).await?;
    let movies_docs: Vec<Movie> = serde_json::from_str(&content)?;

    let task_info = MEILI_CLIENT
        .get()
        .expect("Meilisearch client hasn't been initialized!")
        .index(INDEX_NAME)
        .add_documents(&movies_docs, Some("id"))
        .await?;
    tracing::debug!(?task_info);
    tracing::info!(
        "Data deserialized from {} has been sent to meili server.",
        DATA_FILE_PATH
    );

    Ok(())
}

pub async fn search(query: &str) -> Result<()> {
    use meilisearch_sdk::search::SearchQuery;

    tracing::info!(r#"Searching for "{}"..."#, query);

    let index = MEILI_CLIENT
        .get()
        .expect("Meilisearch client hasn't been initialized!")
        .index(INDEX_NAME);

    let response = SearchQuery::new(&index)
        .with_query(query)
        .execute::<Movie>()
        .await?;

    if response.hits.is_empty() {
        tracing::warn!(r#"No searching result found with query: "{}""#, query);
    } else {
        for item in response.hits.iter() {
            tracing::info!(%item.result);
        }
    }

    Ok(())
}
