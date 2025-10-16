pub use serde::Serialize;

pub use sqlx::{FromRow, PgPool, Result};
#[derive(Debug, Serialize, FromRow)]
pub struct Payment {
    pub id: i64,
    pub tx_hash: String,
    pub log_index: i64,
    pub sender: String,
    pub recipient: String,
    pub amount_text: String,
    pub amount_token: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct NewPayment {
    pub tx_hash: String,
    pub log_index: i64,
    pub sender: String,
    pub recipient: String,
    pub amount_text: String,
    pub amount_token: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub async fn create_pool(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}

pub async fn insert_payment(pool: &PgPool, p: &NewPayment) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO payments (tx_hash, log_index, sender, recipient, amount_text, amount_token, "timestamp")
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (tx_hash, log_index) DO NOTHING
        "#,
    )
    .bind(&p.tx_hash)
    .bind(p.log_index)
    .bind(&p.sender)
    .bind(&p.recipient)
    .bind(&p.amount_text)
    .bind(&p.amount_token)
    .bind(p.timestamp)
    .execute(pool)
    .await?;
    Ok(())
}