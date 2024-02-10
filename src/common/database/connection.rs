use crate::common::config::config::{POSTGRES_CONNECTION_TIMEOUT, POSTGRES_HOST};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub struct DbConnection;

impl DbConnection {
    pub async fn new() -> Result<DatabaseConnection, sea_orm::DbErr> {
        let mut opt = ConnectOptions::new(POSTGRES_HOST.to_string());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(*POSTGRES_CONNECTION_TIMEOUT))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8));

        let db = Database::connect(opt).await?;
        Ok(db)
    }
}
