use serenity::prelude::{TypeMapKey, Mutex};
use std::sync::{Arc};
use serenity::client::bridge::gateway::ShardManager;
use reqwest::Client as ReqwestClient;
use crate::config::ConfigurationData;
use sqlx::PgPool;
use chrono::{DateTime, Utc};

pub struct ShardManagerContainer;
pub struct ConfigContainer;
pub struct DatabasePool;
pub struct ReqwestContainer;
pub struct UptimeContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

impl TypeMapKey for ConfigContainer {
    type Value = ConfigurationData;
}

impl TypeMapKey for DatabasePool {
    type Value = PgPool;
}

impl TypeMapKey for ReqwestContainer {
    type Value = ReqwestClient;
}

impl TypeMapKey for UptimeContainer {
    type Value = DateTime<Utc>;
}