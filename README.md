# Postgres Typescript Generator

Opinionated CLI tool to generate Typescript typings from PostgreSQL databases written in Rust

## Build and tag image

`docker build -t postgres-typescript-generator:latest .`

## Create typings

`docker run --env-file=./.env --network=host -v $(pwd)/psql-typings.toml:/psql-typings.toml -it postgres-typescript-generator:latest`
