use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
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
