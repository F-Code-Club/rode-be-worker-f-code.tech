use sqlx::PgPool;

use crate::config::server::DATABASE_URL;

pub struct AppState {
    pub database: PgPool,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = PgPool::connect(&DATABASE_URL).await?;

        Ok(Self { database: pool })
    }
}
