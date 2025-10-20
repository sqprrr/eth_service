#[derive(Clone)]
pub struct Settings {
    pub database_url: String,
    pub ethereum_ws_url: String,
    pub ethereum_rpc_url: String,
    pub api_listen_address: String,
    pub contract_address: String,
    pub historical_start_block: u64, 
    pub sender_private_key: String,
    pub testnet_rpc_url: String,
    pub testnet_usdc_contract_address: String,
}

pub fn get_settings() -> anyhow::Result<Settings> {
    Ok(Settings {
        database_url: std::env::var("DATABASE_URL")?,
        ethereum_ws_url: std::env::var("ETHEREUM_WS_URL")?,
        ethereum_rpc_url: std::env::var("ETHEREUM_RPC_URL")?,
        api_listen_address: std::env::var("API_LISTEN_ADDRESS")?,
        contract_address: std::env::var("CONTRACT_ADDRESS")?,
        
        historical_start_block: std::env::var("HISTORICAL_START_BLOCK")
            .unwrap_or_else(|_| "0".to_string()) 
            .parse::<u64>()?, 

        sender_private_key: std::env::var("SENDER_PRIVATE_KEY")?,
        testnet_rpc_url: std::env::var("TESTNET_RPC_URL")?,
        testnet_usdc_contract_address: std::env::var("TESTNET_USDC_CONTRACT_ADDRESS")?,
    })
}