# Postgres Typescript Generator

Opinionated CLI tool to generate Typescript typings from PostgreSQL databases written in Rust

## Build, tag and publish image

`make publish`

## Create typings

`docker run --env-file=./.env --network=host -i emarsys/postgres-typescript-generator:latest`

### With extra JSONB typings

`docker run --env-file=./.env --network=host -v $(pwd)/psql-typings.toml:/psql-typings.toml -i emarsys/postgres-typescript-generator:latest`
