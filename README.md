# README

Project to demo [meilisearch](https://github.com/meilisearch/meilisearch) client.

## Prerequisites

[Docker](http://docker.com/) should be installed locally.

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
cargo run
```
