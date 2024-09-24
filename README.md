# README

Project to demo [meilisearch](https://github.com/meilisearch/meilisearch) client.

## Prerequisites

[Docker](http://docker.com/) should be installed locally.\
This demo's `Minimum Supported Rust Version` is `1.70`.

## Meili Server

Be sure to startup meilisearch server before running client.

```sh
# Startup meilisearch server
docker compose up -d
```

After everything, remember to remove the container.

```sh
# Stop meilisearch server and release resources
docker compose down -v
```

## Meili Client

```sh
# Send data to meili server
cargo run -- send
# Search with the given query
cargo run -- search [QUERY]
```

Besides searching in CLI, you can choose to search in a browser.

As long as meili server has been startup and data has been sent,\
you can just go <http://localhost:7700> and search.

## Note

Meilisearch server needs time to index data.\
This may take a few seconds in this demo.

After the indexing, log message can be found in meili server:\
"A batch of tasks was successfully completed."

```sh
# Print meili server logs
docker logs meilisearch
```

As a result, you won't get searching result until the indexing is done.
