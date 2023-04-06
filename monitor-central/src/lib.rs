#[derive(Debug)]
pub struct Stat {
    pub system_identifier: String,
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub uptime: u64,
}
