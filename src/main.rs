use std::{env, str::FromStr};

use once_cell::sync::Lazy;
use sqlx::PgPool;
use tokio::time::interval;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

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
    let result = sqlx::query!(
        r#"
UPDATE scores
SET total_score = score_data.score
FROM (
	SELECT score_id as id, SUM(score) as score
	FROM (
		SELECT score_id, question_id, MAX(score) as score
		FROM submit_histories
		GROUP BY score_id, question_id
	) score_per_question
	GROUP BY score_id
) score_data
WHERE scores.id = score_data.id;
    "#)
    .execute(pool)
    .await?;

    tracing::info!("Updated {} score(s)", result.rows_affected());

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with( tracing_subscriber::fmt::layer().pretty())
        .init();

    let mut interval = interval(std::time::Duration::from_secs(5));
    let pool = PgPool::connect(&DATABASE_URL).await?;

    loop {
        interval.tick().await;

        update_score_table(&pool).await?;
    }
}
