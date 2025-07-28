# Meilisearch Demo

A demo project for exprimenting
[meilisearch](https://github.com/meilisearch/meilisearch) client.

## Prerequisites

- Docker should be installed locally
and make sure `docker compose` is available.
- MSRV (Minimum Supported Rust Version) of this demo is `1.85`.

## Meilisearch Server

Be sure to spin up meilisearch server before running client.

```sh
# Startup meilisearch server.
docker compose up -d
```

After everything, remember to remove the container.

```sh
# Stop meilisearch server and release resources.
docker compose down -v
```

## Meilisearch Client

```sh
# Send data to meili server.
cargo run -- send
# Search with the given query.
cargo run -- search [QUERY]
```

In addition to searching in CLI, you may also choose to search in the browser.
After meilisearch server has spinned up and data has been sent,
visit <http://localhost:7700> to try out searching in the webpage.

## Remark

Meilisearch server needs time to index data,
which may take a few seconds in this demo.
You won't get searching result until the indexing has done.
After the indexing, the following log can be found in meili server:

> "A batch of tasks was successfully completed with 1 successful tasks and 0
> failed tasks."

```sh
# Print and follow meili server logs.
docker compose logs -f meilisearch
```
