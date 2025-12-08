use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum JobStatus {
    Queued,
    Processing,
    Completed,
}

#[derive(Debug, Clone, Serialize)]
pub struct Job {
    pub id: u64,
    pub name: String,
    pub status: JobStatus,
}

#[derive(Deserialize)]
pub struct JobRequest {
    pub name: String,
}