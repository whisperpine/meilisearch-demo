use crate::Movie;
use meilisearch_sdk::client::Client;
use std::sync::LazyLock;

const DATA_FILE_PATH: &str = "movies.json";
const INDEX_NAME: &str = "movies";

static MEILI_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    let meilisearch_url: &str = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    let meilisearch_api_key: &str = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    Client::new(meilisearch_url, Some(meilisearch_api_key))
        .expect("failed to create meilisearch client")
});

/// Read json file from local dir, and send data to meili server.
pub async fn send_data() -> crate::Result<()> {
    let Ok(file) = std::fs::File::open(DATA_FILE_PATH) else {
        return Err(crate::Error::FileNotFound(DATA_FILE_PATH.to_owned()));
    };
    let movies_docs: Vec<Movie> = serde_json::from_reader(file)?;

    tracing::info!("Sending...");
    let task_info = MEILI_CLIENT
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

/// Search with the given query.
pub async fn search(query: &str) -> crate::Result<()> {
    use meilisearch_sdk::search::SearchQuery;

    tracing::info!(r#"Searching for "{}"..."#, query);
    let index = MEILI_CLIENT.index(INDEX_NAME);
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
