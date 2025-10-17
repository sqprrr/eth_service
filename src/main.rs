    
mod api;
mod db;
mod indexer;
mod settings;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let settings = settings::get_settings()?;

    let pool = db::create_pool(&settings.database_url).await?;
    println!("Connected to DB");

    let pool_for_indexer = pool.clone();
    let pool_for_api = pool.clone();
    
    tokio::spawn(async move {
        if let Err(e) = indexer::run_indexer(settings.ethereum_rpc_url, pool_for_indexer).await {
            eprintln!("Fatal error in indexer: {}", e);
        }
    });

    api::run_api_server(pool_for_api, settings.api_listen_address).await;

    Ok(())
}