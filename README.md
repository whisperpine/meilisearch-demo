# README

A demo project for [meilisearch](https://github.com/meilisearch/meilisearch) client.

## Prerequisites

- [Docker](http://docker.com/) should be installed locally
and make sure `docker compose`is available.
- MSRV (Minimum Supported Rust Version) of this demo is `1.85`.

## Meili Server

Be sure to startup meilisearch server before running client.

```sh
# Startup meilisearch server.
docker compose up -d
```

After everything, remember to remove the container.

```sh
# Stop meilisearch server and release resources.
docker compose down -v
```

## Meili Client

```sh
# Send data to meili server.
cargo run -- send
# Search with the given query.
cargo run -- search [QUERY]
```

Besides searching in CLI, you may also choose to search in the browser.\
After meili server has startup and data has been sent,
visit <http://localhost:7700> to search.

## Notes

Meilisearch server needs time to index data,
which may take a few seconds in this demo.\
You won't get searching result until the indexing has done.

After the indexing, log message can be found in meili server:\
"A batch of tasks was successfully completed."

```sh
# Print and follow meili server logs.
docker compose logs -f meilisearch
```
