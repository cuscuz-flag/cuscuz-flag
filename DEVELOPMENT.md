# Development

Most of applications inside of this repository are written in [Rust](https://www.rust-lang.org/) and the best way to install it is using [Rustup](https://rustup.rs/).

## Dashboard && Admin

### Getting started

The dashboard and admin project are based on [Yew](https://yew.rs/), a framework for creating reliable and efficient web application in Rust.

#### Install WebAssembly target

```bash
rustup target add wasm32-unknown-unknown
```

### Install Trunk

```bash
cargo install trunk
```

### Running the project

Inside of dashboard or admin folder, execute:

```bash
trunk serve
```

See [more details](https://trunkrs.dev/configuration/) here to trunk configuration.

## API

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
