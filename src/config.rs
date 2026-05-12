use dotenvy::dotenv;
use std::env;
use anyhow::{Result, Context};

pub struct Config {
    pub manager_db_url: String,
    pub datasaver_db_url: String,
}

pub fn load_config() -> Result<Config> {
    dotenv().ok(); // Ignora el error si no hay archivo .env, usa las del sistema

    Ok(Config {
        manager_db_url: env::var("MANAGER_DB_URL").context("Falta MANAGER_DB_URL")?,
        datasaver_db_url: env::var("DATASAVER_DB_URL").context("Falta DATASAVER_DB_URL")?,
    })
}