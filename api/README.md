# API

API for [admin](../admin) and [dashboard](../dashboard) frontend's

## Getting started

### Database

```bash
docker compose -f infrastructure/docker-compose.yml up -d
```

### Running the project

Inside of `api` folder, execute:

```bash
cargo run
```

### Migrations

You should [install](https://docs.rs/crate/sqlx-cli/latest) `sqlx-cli` to run the commands below

Under `api` folder, you can run the follow commands:

- Create a migration file

```sh
sqlx migrate add <name-of-migration>
```

- Apply a migration

```sh
sqlx migrate run
```

- Destroy the database

```sh
sqlx database drop
```
