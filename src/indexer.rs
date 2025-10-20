use crate::contracts::erc20::ERC20;
use crate::db::NewPayment;
use crate::contracts::erc20::TransferFilter;
use crate::storage::Storage;
use chrono::Utc;
use ethers::prelude::*;
use futures::StreamExt;
use sqlx::PgPool;
use std::sync::Arc;
use ethers::types::H160;
use rust_decimal::Decimal;
use std::str::FromStr;
use ethers::providers::{Provider, Http, Ws};

async fn process_and_save_event(
    storage: &impl Storage,
    transfer_event: TransferFilter,
    meta: LogMeta,
) -> anyhow::Result<()> {
    
let sender: H160 = transfer_event.from;
let recipient: H160 = transfer_event.to;
let value_str = transfer_event.value.to_string();
let value_decimal = Decimal::from_str(&value_str).unwrap();
let amount_token = value_decimal / Decimal::from(1_000_000u64);

    let new_payment = NewPayment {
    block_number: meta.block_number.as_u64() as i64,
    tx_hash: format!("{:?}", meta.transaction_hash),
    log_index: meta.log_index.as_u64() as i64,
    sender: format!("{:?}", sender),
    recipient: format!("{:?}", recipient),
    amount_text: transfer_event.value.to_string(),
    amount_token: Some(format!("{}", amount_token)),
    timestamp: Utc::now(),
};

    // println!(
    //     "Saving Transfer: block={} from={} to={} amount={}",
    //     new_payment.block_number,
    //     new_payment.sender,
    //     new_payment.recipient,
    //     new_payment.amount_token.clone().unwrap()
    // );

    storage.insert_payment(&new_payment).await?;
    Ok(())
}

pub async fn run_indexer(settings: crate::settings::Settings, storage: PgPool) -> anyhow::Result<()> {
    let provider = Arc::new(Provider::<Ws>::connect(&settings.ethereum_ws_url).await?);
    let http_provider = Arc::new(Provider::<Http>::try_from(&settings.ethereum_rpc_url)?);
    let contract_address: H160 = settings.contract_address
    .parse()
    .expect("Invalid contract address");

    let contract = ERC20::new(contract_address, provider.clone());
    let http_contract = ERC20::new(contract_address, http_provider.clone());

    let last_synced_block = storage.get_last_synced_block().await?;
    let from_block = if let Some(last_block_in_db) = last_synced_block {
        let next_block_from_db = (last_block_in_db + 1) as u64;
        let start_block_from_env = settings.historical_start_block;
        let deeper_block = std::cmp::min(next_block_from_db, start_block_from_env);
        println!(
            "DB has data up to block {}. Comparing with ENV start block {}. Resuming from deeper block: {}",
            last_block_in_db, start_block_from_env, deeper_block
        );
        deeper_block
    } else {
        println!(
            "No previous data found. Starting initial sync from block {}",
            settings.historical_start_block
        );
        settings.historical_start_block
    };

    println!("Phase 1: Catching up on any missed blocks...");

    let current_block = http_provider.get_block_number().await?.as_u64();

    if from_block <= current_block {
        const BATCH_SIZE: u64 = 10; 
        let mut current_from = from_block;
        
        while current_from <= current_block {
            let to_block = (current_from + BATCH_SIZE - 1).min(current_block);
            println!("   -> Syncing blocks from {} to {}", current_from, to_block);
            
            let filter = http_contract
                .event::<TransferFilter>()
                .from_block(current_from)
                .to_block(to_block)
                .address(ValueOrArray::Value(contract_address));

            let past_events = filter.query_with_meta().await?;

    
            println!("      Fetched {} events", past_events.len());
            for (event, meta) in past_events {
                if let Err(e) = process_and_save_event(&storage, event, meta).await {
                    eprintln!("Failed to insert historical event: {e}");
                }
            }
            current_from = to_block + 1;
        }
    }
    println!("Phase 1: Historical sync complete!");

    println!("Phase 2: Subscribing to new events...");
    let live_event_filter = contract
        .event::<TransferFilter>() 
        .from_block(current_block + 1);
    let mut stream = live_event_filter.stream_with_meta().await?;
    while let Some(Ok((transfer_event, meta))) = stream.next().await {
        if let Err(e) = process_and_save_event(&storage, transfer_event, meta).await {
            eprintln!("Failed to insert new event: {e}");
        }
    }

    Ok(())
}