use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

use crate::models::JobStatus;
use crate::state::AppState;

fn get_current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub async fn run_worker(worker_id: usize, data: Arc<AppState>) {
    println!("[Worker {}] Started and waiting for jobs...", worker_id);

    loop {
        let next_job_id = {
            let mut queue = data.queue.lock().unwrap();
            queue.pop_front()
        };

        if let Some(id) = next_job_id {
            println!("[Worker {}] Processing job #{}", worker_id, id);

            // Mark as Processing and set started_at timestamp
            {
                let mut jobs = data.jobs.lock().unwrap();
                if let Some(job) = jobs.get_mut(&id) {
                    job.status = JobStatus::Processing;
                    job.started_at = Some(get_current_timestamp());
                }
            }

            // Simulate work
            sleep(Duration::from_secs(5)).await;

            // Simulate 30% failure rate
            let failed = rand::random::<f64>() < 0.3;

            {
                let mut jobs = data.jobs.lock().unwrap();
                if let Some(job) = jobs.get_mut(&id) {
                    if failed {
                        // Job failed
                        job.status = JobStatus::Failed;
                        job.failed_reason = Some("Simulated random failure".to_string());
                        println!("[Worker {}] Job #{} failed! (retry {}/{})", worker_id, id, job.retry_count, job.max_retries);

                        // Retry logic: if under max_retries, re-queue the job
                        if job.retry_count < job.max_retries {
                            job.retry_count += 1;
                            job.status = JobStatus::Queued;
                            job.started_at = None;
                            job.failed_reason = None;

                            // Re-enqueue for retry
                            let mut queue = data.queue.lock().unwrap();
                            queue.push_back(id);
                            println!("[Worker {}] Job #{} re-queued for retry {}/{}", worker_id, id, job.retry_count, job.max_retries);
                        } else {
                            println!("[Worker {}] Job #{} permanently failed after {} retries", worker_id, id, job.retry_count);
                        }
                    } else {
                        // Job succeeded
                        job.status = JobStatus::Completed;
                        job.completed_at = Some(get_current_timestamp());
                        println!("[Worker {}] Job #{} completed successfully!", worker_id, id);
                    }
                }
            }
        } else {
            sleep(Duration::from_millis(500)).await;
        }
    }
}
