use axum;
use common::database::connection::db_connection;
use presentation::routers;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod common;
mod infrastructure;
mod presentation;
mod usecase;
#[derive(Debug, Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_web_server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = db_connection("postgresql://test:test@rust_db:5432/test".to_string()).await.unwrap();

    let app_state = Arc::new(AppState { db });
    let app = routers::create_router(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
