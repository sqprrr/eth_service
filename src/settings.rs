
pub struct Settings {
    pub database_url: String,
    pub ethereum_rpc_url: String,
    pub api_listen_address: String,
}

pub fn get_settings() -> anyhow::Result<Settings> {
    Ok(Settings {
        database_url: std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set"),
        ethereum_rpc_url: std::env::var("ETHEREUM_RPC_URL")
            .expect("ETHEREUM_RPC_URL must be set"),
        api_listen_address: std::env::var("API_LISTEN_ADDRESS")
            .expect("API_LISTEN_ADDRESS must be set"),
    })
}