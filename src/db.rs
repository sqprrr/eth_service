pub use serde::Serialize;
use crate::storage::Storage;
use async_trait::async_trait;
pub use sqlx::{FromRow, PgPool, Row};
use crate::api_models::ListPaymentsParams;


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

    async fn get_payment_by_id(&self, id: i64) -> anyhow::Result<Option<Payment>> {
        let payment = sqlx::query_as("SELECT * FROM payments WHERE id = $1")
            .bind(id)
            .fetch_optional(self) 
            .await?; 

        Ok(payment) 
    }

        async fn list_payments(&self, params: &ListPaymentsParams) -> anyhow::Result<Vec<Payment>> {
        use chrono::{TimeZone, Utc};
        use sqlx::QueryBuilder;

        let mut query_builder = QueryBuilder::new("SELECT * FROM payments WHERE 1=1");

        if let Some(sender) = &params.sender {
            query_builder.push(" AND sender = ").push_bind(sender);
        }
        if let Some(recipient) = &params.recipient {
            query_builder.push(" AND recipient = ").push_bind(recipient);
        }
        if let Some(participant) = &params.participant {
            query_builder
                .push(" AND (sender = ")
                .push_bind(participant.clone())
                .push(" OR recipient = ")
                .push_bind(participant)
                .push(")");
        }
        if let Some(ts) = params.created_after {
            if let Some(dt) = Utc.timestamp_opt(ts, 0).single() {
                query_builder.push(" AND \"timestamp\" > ").push_bind(dt);
            }
        }
        if let Some(ts) = params.created_before {
            if let Some(dt) = Utc.timestamp_opt(ts, 0).single() {
                query_builder.push(" AND \"timestamp\" < ").push_bind(dt);
            }
        }

        query_builder.push(" ORDER BY block_number DESC LIMIT 100");

        let payments = query_builder.build_query_as::<Payment>().fetch_all(self).await?;
        Ok(payments)
    }
}