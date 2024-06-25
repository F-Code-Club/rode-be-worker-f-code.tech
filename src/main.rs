use std::{env, str::FromStr};

use once_cell::sync::Lazy;
use sqlx::PgPool;
use tokio::time::interval;

pub fn env_or_default<T: FromStr>(env_name: &'static str, default: T) -> T {
    match env::var(env_name) {
        Err(_) => default,
        Ok(raw) => match raw.parse() {
            Ok(value) => value,
            Err(_) => default,
        },
    }
}

pub static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    env_or_default(
        "DATABASE_URL",
        "postgres://user:password@host/database".to_string(),
    )
});

async fn update_score_table(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        UPDATE scores
        SET total_score = COALESCE(
        (SELECT SUM(submit_histories.score)
        FROM submit_histories
        WHERE submit_histories.score_id = scores.id
        GROUP BY scores.id),
        0
        );
    "#
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut interval = interval(std::time::Duration::from_secs(5));
    let pool = PgPool::connect(&DATABASE_URL).await?;

    loop {
        interval.tick().await;

        update_score_table(&pool).await?;
    }
}
