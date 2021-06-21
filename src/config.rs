use serde::{Deserialize, Serialize};


/**
  * Main Configuration struct
 **/
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigurationData {
    pub bot: BotConfig,
    pub api: APIConfig,
    pub database: DatabaseConfig
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotConfig {
    pub prefix: String,
    pub owners: Vec<String>,
    pub application: BotApplicationConfig,
    pub logging: BotLoggingConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotLoggingConfig {
    pub enabled: bool,
    pub level: String,
    pub stats_channel: u64,
    pub stats_message: u64
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotApplicationConfig {
    pub id: u64,
    pub token: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIConfig {
    pub hypixel: HypixelAPIConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HypixelAPIConfig {
    pub key: String,
}


