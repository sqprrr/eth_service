
    mod api;
    mod config;
    mod db;
    mod indexer;


    #[tokio::main]
    async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

 
        let config = config::load_config()?;
    
        let pool = db::create_pool(&config.database.url).await?;
        println!("Connected to DB");

        let pool_for_indexer = pool.clone();
        let pool_for_api = pool.clone();
        let rpc_url_for_indexer = config.ethereum.rpc_url.clone();

        tokio::spawn(async move {
            if let Err(e) = indexer::run_indexer(rpc_url_for_indexer, pool_for_indexer).await {
                eprintln!("Fatal error in indexer: {}", e);
            }
        });

        api::run_api_server(pool_for_api, config.api.listen_address).await;

        Ok(())
    }