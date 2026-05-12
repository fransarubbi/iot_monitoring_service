mod config;
mod db;
mod models;
mod monitor;
mod telegram;

use tracing::{info, error};
use crate::telegram::TelegramNotifier;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    tracing_subscriber::fmt::init();
    info!("Info: iniciando servicio IoT Monitoring...");

    let config = config::load_config()?;
    let telegram = TelegramNotifier::new().expect("Error crítico: no se pudieron cargar las credenciales de Telegram");

    let manager_pool = db::connect(&config.manager_db_url).await?;
    let datasaver_pool = db::connect(&config.datasaver_db_url).await?;
    
    if let Err(e) = monitor::start_watchdog(
        manager_pool,
        datasaver_pool,
        telegram
    ).await {
        error!("Error crítico en el loop principal: {:?}", e);
    }

    Ok(())
}