use dotenvy::dotenv;
use std::env;
use anyhow::{Result, Context};

pub struct Config {
    pub manager_db_url: String,
    pub datasaver_db_url: String,
}

pub fn load_config() -> Result<Config> {
    dotenv().ok(); 

    Ok(Config {
        manager_db_url: env::var("MANAGER_DB_URL").context("Error: falta MANAGER_DB_URL en .env")?,
        datasaver_db_url: env::var("DATASAVER_DB_URL").context("Error: falta DATASAVER_DB_URL en .env")?,
    })
}