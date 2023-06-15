use anyhow::Result;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<()> {
    write_data().await?;
    Ok(())
}

/// Read json file from local dir, and send data to meili server.
pub async fn write_data() -> Result<()> {
    use meilisearch_sdk::client::Client;
    use tokio::fs;

    let client = Client::new("http://localhost:7700", Some("master-key"));
    let content = fs::read_to_string("movies.json").await?;
    let movies_docs: Vec<Movie> = serde_json::from_str(&content)?;

    client
        .index("movies")
        .add_documents(&movies_docs, Some("id"))
        .await?;

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
