use sqlx::FromRow;


#[derive(FromRow, Debug)]
pub struct EdgeConfig {
    pub edge_id: String,
    pub name: String,
}


#[derive(FromRow, Debug)]
pub struct HubConfig {
    pub hub_id: String,
    pub network_id: String,
    pub device_name: Option<String>,
    pub sample: i64,
}


#[derive(FromRow, Debug)]
pub struct LastSeenRecord {
    pub last_seen: Option<chrono::DateTime<chrono::Utc>>,
}