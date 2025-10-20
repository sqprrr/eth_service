use crate::db::{NewPayment};
use async_trait::async_trait;

#[async_trait]
pub trait Storage {
    async fn insert_payment(&self, payment: &NewPayment) -> anyhow::Result<()>;
    
    async fn get_last_synced_block(&self) -> anyhow::Result<Option<i64>>;

    // async fn get_payment_by_id(&self, id: i64) -> anyhow::Result<Option<Payment>>;
}