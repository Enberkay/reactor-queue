use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum JobStatus {
    Queued,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize)]
pub struct Job {
    pub id: u64,
    pub name: String,
    pub status: JobStatus,
    pub retry_count: u32,
    pub max_retries: u32,
    pub created_at: u64,        // Unix timestamp
    pub started_at: Option<u64>, // Unix timestamp
    pub completed_at: Option<u64>, // Unix timestamp
    pub failed_reason: Option<String>,
}

#[derive(Deserialize)]
pub struct JobRequest {
    pub name: String,
}