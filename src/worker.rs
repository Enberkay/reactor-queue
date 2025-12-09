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

pub async fn run_worker(data: Arc<AppState>) {
    println!("Worker started and waiting for jobs...");

    loop {
        let next_job_id = {
            let mut queue = data.queue.lock().unwrap();
            queue.pop_front()
        };

        if let Some(id) = next_job_id {
            println!("Processing job #{}", id);

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
                        println!("Job #{} failed! (retry {}/{})", id, job.retry_count, job.max_retries);

                        // Retry logic: if under max_retries, re-queue the job
                        if job.retry_count < job.max_retries {
                            job.retry_count += 1;
                            job.status = JobStatus::Queued;
                            job.started_at = None;
                            job.failed_reason = None;

                            // Re-enqueue for retry
                            let mut queue = data.queue.lock().unwrap();
                            queue.push_back(id);
                            println!("Job #{} re-queued for retry {}/{}", id, job.retry_count, job.max_retries);
                        } else {
                            println!("Job #{} permanently failed after {} retries", id, job.retry_count);
                        }
                    } else {
                        // Job succeeded
                        job.status = JobStatus::Completed;
                        job.completed_at = Some(get_current_timestamp());
                        println!("Job #{} completed successfully!", id);
                    }
                }
            }
        } else {
            sleep(Duration::from_millis(500)).await;
        }
    }
}
