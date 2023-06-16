use anyhow::Result;
use meilisearch_sdk::client::Client;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

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

#[tokio::main]
async fn main() -> Result<()> {
    println!("CARGO_PKG_NAME: {}", CARGO_PKG_NAME);
    init_tracing().await;
    init_meili_client();
    send_data().await?;
    search("botman").await?;
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

    tracing::info!("CARGO_PKG_NAME: {}", CARGO_PKG_NAME);
    tracing::info!("CARGO_PKG_VERSION: {}", CARGO_PKG_VERSION);
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

    for item in response.hits.iter() {
        tracing::info!(%item.result);
    }

    Ok(())
}
