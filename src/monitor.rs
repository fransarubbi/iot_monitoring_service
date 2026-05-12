use sqlx::PgPool;
use tokio::time::{interval, Duration};
use tracing::{debug, info, warn};
use crate::{config::Config};
use crate::telegram::TelegramNotifier;


pub async fn start_watchdog(manager_pool: PgPool,
                            datasaver_pool: PgPool,
                            config: Config,
                            telegram: TelegramNotifier
) -> anyhow::Result<()> {
    let mut ticker = interval(Duration::from_secs(30));

    loop {
        ticker.tick().await;  // Espera hasta el próximo ciclo
        debug!("Debug: ejecutando revisión de estado de red...");

        if let Err(e) = check_edges(&manager_pool, &datasaver_pool, &config).await {
            warn!("Error al revisar edges: {}", e);
        }

        if let Err(e) = check_hubs(&manager_pool, &datasaver_pool, &config).await {
            warn!("Error al revisar hubs: {}", e);
        }
    }
}

async fn check_edges(_m_pool: &PgPool, _d_pool: &PgPool, _config: &Config) -> anyhow::Result<()> {
    // Aquí implementás las consultas SQL (SELECT MAX(timestamp)...)
    // Si detectás una caída:
    // telegram::send_alert(config, "🚨 Edge 'XYZ' caído!").await?;
    Ok(())
}

async fn check_hubs(_m_pool: &PgPool, _d_pool: &PgPool, _config: &Config) -> anyhow::Result<()> {
    // Lógica similar para los Hubs
    Ok(())
}