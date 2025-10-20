use crate::settings::Settings;
use anyhow::Context;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;
use crate::contracts::erc20::ERC20;


pub async fn send_usdc_transfer(
    settings: &Settings,
    recipient_address: String,
    amount_str: String,
) -> anyhow::Result<String> {

    let provider = Provider::<Http>::try_from(&settings.testnet_rpc_url)
        .context("Failed to connect to testnet provider")?;
    let provider = Arc::new(provider);

    let wallet: LocalWallet = settings.sender_private_key.parse()
        .context("Failed to parse private key")?;
    let chain_id = provider.get_chainid().await?.as_u64();

    let client = SignerMiddleware::new(provider, wallet.with_chain_id(chain_id));
    let client = Arc::new(client);

    let contract_address = Address::from_str(&settings.testnet_usdc_contract_address)?;
    let contract = ERC20::new(contract_address, client);

    let to_address = Address::from_str(&recipient_address)?;
    let amount_u256: U256 = ethers::utils::parse_units(&amount_str, 6)?
    .into();

    println!("Sending {} USDC to {}", amount_str, recipient_address);
    let tx = contract.transfer(to_address, amount_u256);
    let pending_tx = tx.send().await.context("Failed to send transaction")?;
    
    let receipt = pending_tx.await
        .context("Failed to get transaction receipt")?
        .context("Transaction was dropped from the mempool")?;

    let tx_hash = format!("{:?}", receipt.transaction_hash);
    println!("Transaction successfully sent! Hash: {}", tx_hash);

    Ok(tx_hash)
}