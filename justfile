# list all available subcommands
_default:
  @just --list

# show the help message of meilisearch-demo
help:
  cargo run -- help

# send data to meili server by meilisearch-demo
send:
  cargo run -- send

# search with the given query by meilisearch-demo
search *PATTERN:
  cargo run -- search {{PATTERN}}

# check health of the meilisearch server
health:
  curl http://localhost:7700/health
