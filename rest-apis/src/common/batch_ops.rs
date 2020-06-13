use crate::common::AppError;
use rusoto_batch::{Batch, BatchClient, SubmitJobRequest};
use rusoto_core::Region;
use std::error::Error;

/** Submits batch job. */
pub async fn submit_batch_job(
    region: Region,
    job_queue: String,
    job_definition: String,
    job_name: String,
) -> Result<String, AppError> {
    BatchClient::new(region)
        .submit_job(SubmitJobRequest {
            job_queue,
            job_definition,
            job_name,
            ..SubmitJobRequest::default()
        })
        .await
        .map_err(|err| AppError {
            code: "".to_string(),
            message: "cannot create batch job".to_string(),
            details: err
                .source()
                .map(|err_src| format!("{}", err_src))
                .unwrap_or_else(|| "unknown".to_string()),
        })
        .map(|response| response.job_id)
}
