use actix_web::{web, HttpResponse, Responder};
use std::sync::{atomic::Ordering, Arc};

use crate::models::{Job, JobRequest, JobStatus};
use crate::state::AppState;

// POST /jobs
pub async fn submit_job(
    data: web::Data<Arc<AppState>>,
    req: web::Json<JobRequest>
) -> impl Responder {
    let id = data.job_counter.fetch_add(1, Ordering::SeqCst);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let new_job = Job {
        id,
        name: req.name.clone(),
        status: JobStatus::Queued,
        retry_count: 0,
        max_retries: 3,
        created_at: now,
        started_at: None,
        completed_at: None,
        failed_reason: None,
    };

    {
        let mut jobs = data.jobs.lock().unwrap();
        let mut queue = data.queue.lock().unwrap();
        jobs.insert(id, new_job.clone());
        queue.push_back(id);
    }

    println!("Received Job #{} ({})", id, req.name);
    HttpResponse::Ok().json(new_job)
}

// GET /jobs/{id}
pub async fn get_job(
    data: web::Data<Arc<AppState>>, 
    path: web::Path<u64>
) -> impl Responder {
    let id = path.into_inner();
    let jobs = data.jobs.lock().unwrap();

    match jobs.get(&id) {
        Some(job) => HttpResponse::Ok().json(job),
        None => HttpResponse::NotFound().body("Job not found"),
    }
}