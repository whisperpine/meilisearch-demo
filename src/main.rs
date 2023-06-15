use anyhow::Result;
use serde::{Deserialize, Serialize};

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<()> {
    println!("CARGO_PKG_NAME: {}", CARGO_PKG_NAME);
    init_tracing().await;
    // write_data().await?;
    Ok(())
}

async fn init_tracing() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("CARGO_PKG_NAME: {}", CARGO_PKG_NAME);
    tracing::info!("CARGO_PKG_VERSION: {}", CARGO_PKG_VERSION);
}

/// Read json file from local dir, and send data to meili server.
pub async fn write_data() -> Result<()> {
    use meilisearch_sdk::client::Client;
    use tokio::fs;

    let client = Client::new("http://localhost:7700", Some("master-key"));
    let content = fs::read_to_string("movies.json").await?;
    let movies_docs: Vec<Movie> = serde_json::from_str(&content)?;

    let task_info = client
        .index("movies")
        .add_documents(&movies_docs, Some("id"))
        .await?;

    tracing::debug!(?task_info);

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Movie {
    id: i64,
    title: String,
    poster: String,
    overview: String,
    release_date: i64,
    genres: Vec<String>,
}
