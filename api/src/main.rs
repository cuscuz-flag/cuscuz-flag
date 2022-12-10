use std::net::SocketAddr;

use anyhow::{Context, Result};
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};

mod error;
mod handlers;

pub use handlers::{signin, signup};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = dotenvy::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await?;

    let app = app(pool).await?;
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn app(pool: Pool<Postgres>) -> Result<Router, Box<dyn std::error::Error>> {
    // TODO: create a service to run all migrations before the server starts.

    Ok(Router::new()
        .route("/sign-up", post(signup))
        .route("/sign-in", post(signin))
        .route("/ping", get(|| async { "pong" }))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any),
        )
        .with_state(pool))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl-C handler")
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install Ctrl-C handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c =>{},
        _ = terminate =>{},
    }

    println!("signal received, starting graceful shutdown")
}
