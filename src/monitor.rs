use sqlx::PgPool;
use tokio::time::{interval, Duration};
use tracing::{debug, info, warn, error};
use std::collections::HashSet;
use chrono::Utc;
use crate::telegram::TelegramNotifier;
use crate::models::{EdgeConfig, HubConfig, LastSeenRecord};


pub async fn start_watchdog(manager_pool: PgPool,
                            datasaver_pool: PgPool,
                            telegram: TelegramNotifier,
) -> anyhow::Result<()> {

    // Intervalo de revisión cada 30 segundos
    let mut ticker = interval(Duration::from_secs(30));

    // Estados en memoria para evitar el spamming en Telegram
    let mut alerted_edges: HashSet<String> = HashSet::new();
    let mut alerted_hubs: HashSet<String> = HashSet::new();

    info!("Info: inició el loop principal del Watchdog.");

    loop {
        ticker.tick().await;
        debug!("Debug: ejecutando revisión de estado de red...");

        if let Err(e) = check_edges(&manager_pool, &datasaver_pool, &telegram, &mut alerted_edges).await {
            error!("Error crítico al revisar edges: {}", e);
        }

        if let Err(e) = check_hubs(&manager_pool, &datasaver_pool, &telegram, &mut alerted_hubs).await {
            error!("Error crítico al revisar hubs: {}", e);
        }
    }
}


async fn check_edges(m_pool: &PgPool,
                     d_pool: &PgPool,
                     telegram: &TelegramNotifier,
                     alerted: &mut HashSet<String>,
) -> anyhow::Result<()> {

    let edges = sqlx::query_as::<_, EdgeConfig>(
        "SELECT edge_id, name FROM edges"
    )
        .fetch_all(m_pool)
        .await?;

    let active_edge_ids: HashSet<&String> = edges.iter().map(|e| &e.edge_id).collect();
    alerted.retain(|id| active_edge_ids.contains(id));

    let now = Utc::now();

    for edge in edges {
        // Buscamos el último reporte real en datasaver_db
        let record = sqlx::query_as::<_, LastSeenRecord>(
            "SELECT MAX(timestamp) as last_seen FROM metric WHERE sender_user_id = $1"
        )
            .bind(&edge.edge_id)
            .fetch_one(d_pool)
            .await?;

        if let Some(last_seen) = record.last_seen {
            let elapsed_seconds = now.signed_duration_since(last_seen).num_seconds();

            // Le damos un margen de gracia de 20s por posibles latencias de red o de ingesta
            if elapsed_seconds > (120 + 20) {
                if !alerted.contains(&edge.edge_id) {
                    let msg = format!(
                        "🚨 *ALERTA CRÍTICA: Edge Caído*\n\n*Nombre:* {}\n*ID:* `{}`\n*Inactivo por:* {} segundos\n*Tolerancia:* {} segundos",
                        edge.name, edge.edge_id, elapsed_seconds, 120
                    );
                    telegram.send_alert(&msg).await;
                    alerted.insert(edge.edge_id.clone());
                    warn!("Warning: edge {} caído. Alerta enviada.", edge.edge_id);
                }
            } else {
                if alerted.contains(&edge.edge_id) {
                    let msg = format!(
                        "✅ *Edge Recuperado*\n\n*Nombre:* {}\nHa vuelto a enviar métricas correctamente.",
                        edge.name
                    );
                    telegram.send_alert(&msg).await;
                    alerted.remove(&edge.edge_id);
                    info!("Info: edge {} recuperado.", edge.edge_id);
                }
            }
        }
    }
    Ok(())
}


async fn check_hubs(m_pool: &PgPool,
                    d_pool: &PgPool,
                    telegram: &TelegramNotifier,
                    alerted: &mut HashSet<String>,
) -> anyhow::Result<()> {

    // Obtenemos solo los Hubs que pertenecen a redes activas
    let hubs = sqlx::query_as::<_, HubConfig>(
        r#"
        SELECT h.hub_id, h.network_id, h.device_name
        FROM hubs h
        INNER JOIN networks n ON h.network_id = n.network_id
        WHERE n.active = true
        "#
    )
        .fetch_all(m_pool)
        .await?;

    let active_hub_ids: HashSet<&String> = hubs.iter().map(|h| &h.hub_id).collect();
    // Eliminamos del registro en memoria cualquier hub que ya no esté en la BD o esté inactivo
    alerted.retain(|id| active_hub_ids.contains(id));

    let now = Utc::now();

    for hub in hubs {
        // Monitoreamos a través de la tabla `monitor` usando `sender_user_id`
        let record = sqlx::query_as::<_, LastSeenRecord>(
            "SELECT MAX(timestamp) as last_seen FROM monitor WHERE sender_user_id = $1"
        )
            .bind(&hub.hub_id)
            .fetch_one(d_pool)
            .await?;

        let dev_name = hub.device_name.unwrap_or_else(|| "Desconocido".to_string());

        if let Some(last_seen) = record.last_seen {
            let elapsed_seconds = now.signed_duration_since(last_seen).num_seconds();

            if elapsed_seconds > hub.sample*2 + 20 {
                if !alerted.contains(&hub.hub_id) {
                    let msg = format!(
                        "⚠️ *ALERTA: Hub Inactivo*\n\n*Dispositivo:* {}\n*Network ID:* `{}`\n*Hub ID:* `{}`\n*Inactivo por:* {} segundos",
                        dev_name, hub.network_id, hub.hub_id, elapsed_seconds
                    );
                    telegram.send_alert(&msg).await;
                    alerted.insert(hub.hub_id.clone());
                    warn!("Warning: Hub {} inactivo. Alerta enviada.", hub.hub_id);
                }
            } else {
                if alerted.contains(&hub.hub_id) {
                    let msg = format!(
                        "✅ *Hub Recuperado*\n\n*Dispositivo:* {}\nEl dispositivo MQTT está en línea nuevamente.",
                        dev_name
                    );
                    telegram.send_alert(&msg).await;
                    alerted.remove(&hub.hub_id);
                    info!("Info: Hub {} recuperado.", hub.hub_id);
                }
            }
        }
    }
    Ok(())
}