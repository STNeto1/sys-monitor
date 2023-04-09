use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct DBStat {
    pub id: String,
    pub system_identifier: String,
    pub total_memory: i64,
    pub used_memory: i64,
    pub total_swap: i64,
    pub used_swap: i64,
    pub uptime: i64,
    pub timestamp: String,
}
