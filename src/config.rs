
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ethereum: EthereumConfig,
    pub api: ApiConfig,
}


#[derive(Debug, Deserialize)]
pub struct EthereumConfig { pub rpc_url: String }
#[derive(Debug, Deserialize)]
pub struct ApiConfig { pub listen_address: String }

pub fn load_config() -> anyhow::Result<Config> {
 
    dotenvy::dotenv().ok();

    let settings = config::Config::builder()

        .add_source(config::File::with_name("config").required(false))

        .add_source(
            config::Environment::default()
                .try_parsing(true)
                .separator("__") 
        )
        .build()?;

    Ok(settings.try_deserialize()?)
}