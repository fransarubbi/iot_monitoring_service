use std::env;
use reqwest::Client;
use serde_json::json;
use tracing::{error, info};


/// Estructura para manejar el cliente de Telegram de forma reutilizable.
#[derive(Clone, Debug)]
pub struct TelegramNotifier {
    client: Client,
    bot_token: String,
    chat_id: String,
}

impl TelegramNotifier {
    /// Inicializa el notificador.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {

        info!("Info: creando objeto telegram_notifier");

        Ok(TelegramNotifier {
            client: Client::new(),
            bot_token: env::var("BOT_TOKEN")
                .expect("BOT_TOKEN no está configurado"),
            chat_id: env::var("CHAT_ID")
                .expect("CHAT_ID no está configurado"),
        })
    }

    /// Función asíncrona para enviar la alerta.
    pub async fn send_alert(&self, message: &str) {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token);

        let payload = json!({
            "chat_id": self.chat_id,
            "text": message,
            "parse_mode": "Markdown"
        });

        match self.client.post(&url).json(&payload).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    info!("Info: alerta de Telegram enviada con éxito");
                } else {
                    error!("Error: fallo al enviar alerta, código de estado: {}", response.status());
                }
            }
            Err(e) => {
                error!("Error: fallo en la red al intentar enviar alerta a Telegram: {}", e);
            }
        }
    }
}