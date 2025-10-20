mod api;
mod api_models;
mod db;
mod indexer;
mod settings;
mod state;
mod storage;
mod tx_sender;
pub mod contracts;

use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let settings = settings::get_settings()?;

    let pool = db::create_pool(&settings.database_url).await?;
    println!("Connected to DB");

    let app_state = AppState {
        pool: pool.clone(),
        settings: settings.clone(),
    };
    
    tokio::spawn(async move {
        if let Err(e) = indexer::run_indexer(settings, pool).await {
            eprintln!("Fatal error in indexer: {}", e);
        }
    });

    api::run_api_server(app_state.clone()).await;


    Ok(())
}