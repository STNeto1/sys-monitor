use std::env;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    pub system_identifier: String,
    pub total_memory: i64,
    pub used_memory: i64,
    pub total_swap: i64,
    pub used_swap: i64,
    pub uptime: i64,
    pub timestamp: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DBStat {
    pub id: Uuid,
    pub system_identifier: String,
    pub total_memory: i64,
    pub used_memory: i64,
    pub total_swap: i64,
    pub used_swap: i64,
    pub uptime: i64,
    pub timestamp: String,
}

pub fn get_from_env(key: &str) -> String {
    env::var(key).expect(&format!("{} must be set", key))
}
