//! A demo project for meilisearch client.

#![cfg_attr(
    not(debug_assertions),
    deny(warnings, missing_docs),
    deny(clippy::todo, clippy::unwrap_used)
)]
#![cfg_attr(
    not(any(test, debug_assertions)),
    deny(clippy::print_stdout, clippy::dbg_macro)
)]

mod cli;
mod error;
mod meili;
mod movie;

pub use cli::app;
pub use error::{Error, Result};
pub use meili::{search, send_data};
pub use movie::Movie;

#[tokio::main]
async fn main() {
    init_tracing_subscriber();
    cli::app().await.unwrap_or_else(|x| tracing::error!("{x}"));
}

fn init_tracing_subscriber() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
