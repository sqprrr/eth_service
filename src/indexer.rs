use crate::db::{insert_payment, NewPayment};
use ethers::prelude::*;
use futures::StreamExt;
use sqlx::PgPool;
use std::sync::Arc;
use chrono::Utc;

pub async fn run_indexer(rpc_url: String, pool: PgPool) -> anyhow::Result<()> {
    let provider = Arc::new(Provider::<Ws>::connect(&rpc_url).await?);
    let contract_address: Address = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".parse()?;


    abigen!(
        ERC20,
        r#"[
            event Transfer(address indexed from, address indexed to, uint256 value)
        ]"#
    );

    let contract = ERC20::new(contract_address, provider);

    let event_filter = contract.event::<TransferFilter>();
    let mut stream = event_filter.stream_with_meta().await?;

    println!("Subscribed to USDC Transfer events...");

    while let Some(Ok((transfer_event, meta))) = stream.next().await {
        let tx_hash = format!("{:?}", meta.transaction_hash);
        let log_index = meta.log_index.as_u64() as i64;
        let amount_token = format!("{}", transfer_event.value.as_u128() as f64 / 1_000_000f64);

        let new_payment = NewPayment {
            tx_hash,
            log_index,
            sender: format!("{:?}", transfer_event.from),
            recipient: format!("{:?}", transfer_event.to),
            amount_text: transfer_event.value.to_string(),
            amount_token: Some(amount_token),
            timestamp: Utc::now(),
        };
        println!(
            "New Transfer: from={} to={} amount={}",
            new_payment.sender, new_payment.recipient, new_payment.amount_token.clone().unwrap()
        );

        if let Err(e) = insert_payment(&pool, &new_payment).await {
            eprintln!(" Failed to insert: {e}");
        }
    }
    Ok(())
}
