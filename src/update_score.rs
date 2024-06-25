use sqlx::PgPool;

pub async fn update_score_tables(pool: &PgPool) -> anyhow::Result<()> {
    update_score_tables_internal(pool).await?;

    Ok(())
}
async fn update_score_tables_internal(pool: &PgPool) -> anyhow::Result<()> {
    let list_score = Score::get_all_score_id(pool).await?;

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
