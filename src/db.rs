pub use serde::Serialize;
use crate::storage::Storage;
use async_trait::async_trait;
pub use sqlx::{FromRow, PgPool, Row};


#[derive(Debug, Serialize, FromRow)]
pub struct Payment {
    pub id: i64,
    pub block_number: i64,
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
    pub block_number: i64,
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

#[async_trait]
impl Storage for PgPool {
    async fn insert_payment(&self, p: &NewPayment) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO payments (block_number, tx_hash, log_index, sender, recipient, amount_text, amount_token, "timestamp")
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (tx_hash, log_index) DO NOTHING
            "#,
        )
        .bind(p.block_number)
        .bind(&p.tx_hash)
        .bind(p.log_index)
        .bind(&p.sender)
        .bind(&p.recipient)
        .bind(&p.amount_text)
        .bind(&p.amount_token)
        .bind(p.timestamp)
        .execute(self) 
        .await?;
        Ok(())
    }

    async fn get_last_synced_block(&self) -> anyhow::Result<Option<i64>> {
        let row = sqlx::query("SELECT MAX(block_number) FROM payments")
            .fetch_optional(self) 
            .await?;
    
        match row {
            Some(row) => Ok(row.get(0)),
            None => Ok(None),
        }
    }
}