use std::sync::Arc;
use anyhow::Result;
use chrono::{DateTime, Utc};
use tracing::{info, warn, error};
use serde_json;

use crate::queues::domain::entities::{Job, JobStatus, FailedJob};
use crate::queues::domain::repositories::{JobRepository, FailedJobRepository, QueueStats};
use crate::queues::domain::value_objects::QueueName;

/// Сервис для управления задачами в очереди
pub struct JobService {
    job_repository: Arc<dyn JobRepository>,
    failed_job_repository: Arc<dyn FailedJobRepository>,
}

impl JobService {
    pub fn new(
        job_repository: Arc<dyn JobRepository>,
        failed_job_repository: Arc<dyn FailedJobRepository>,
    ) -> Self {
        Self {
            job_repository,
            failed_job_repository,
        }
    }

    /// Добавить новую задачу в очередь
    pub async fn enqueue(&self, mut job: Job) -> Result<Job> {
        // Валидация payload как JSON
        if !job.payload.is_empty() {
            serde_json::from_str::<serde_json::Value>(&job.payload)
                .map_err(|e| anyhow::anyhow!("Invalid JSON payload: {}", e))?;
        }

        info!(
            "Enqueuing job to queue '{}' with payload length: {}", 
            job.queue_name, 
            job.payload.len()
        );

        let created_job = self.job_repository.create(&job).await?;
        
        info!("Successfully enqueued job with ID: {:?}", created_job.id);
        Ok(created_job)
    }

    /// Добавить отложенную задачу
    pub async fn enqueue_delayed(
        &self, 
        queue_name: QueueName, 
        payload: String, 
        scheduled_at: DateTime<Utc>
    ) -> Result<Job> {
        let job = Job::new_delayed(queue_name, payload, scheduled_at);
        self.enqueue(job).await
    }

    /// Получить следующую задачу для выполнения
    pub async fn get_next_job(&self, queue_name: &QueueName) -> Result<Option<Job>> {
        let job = self.job_repository.get_next_pending_job(queue_name).await?;
        
        if let Some(ref job) = job {
            info!("Retrieved next job from queue '{}': ID {:?}", queue_name, job.id);
        }
        
        Ok(job)
    }

    /// Начать выполнение задачи
    pub async fn start_job(&self, mut job: Job) -> Result<Job> {
        if !job.is_ready_to_execute() {
            return Err(anyhow::anyhow!("Job is not ready to execute"));
        }

        job.start_execution()?;
        
        info!("Starting job execution: ID {:?}, attempt {}", job.id, job.attempts);
        
        let updated_job = self.job_repository.update(&job).await?;
        Ok(updated_job)
    }

    /// Завершить задачу как успешную
    pub async fn complete_job(&self, mut job: Job) -> Result<Job> {
        job.mark_completed();
        
        info!("Completing job: ID {:?}", job.id);
        
        let updated_job = self.job_repository.update(&job).await?;
        Ok(updated_job)
    }

    /// Завершить задачу как проваленную
    pub async fn fail_job(&self, mut job: Job, error_message: String) -> Result<()> {
        error!("Failing job ID {:?}: {}", job.id, error_message);
        
        job.mark_failed();
        
        // Если превышено максимальное количество попыток, перемещаем в failed_jobs
        if job.is_max_attempts_exceeded() {
            warn!(
                "Job ID {:?} exceeded max attempts ({}), moving to failed_jobs", 
                job.id, 
                job.max_attempts
            );
            
            let failed_job = FailedJob::from_job(job.clone(), error_message);
            self.failed_job_repository.create(&failed_job).await?;
            
            // Удаляем из основной таблицы jobs
            if let Some(id) = job.id {
                self.job_repository.delete(id).await?;
            }
        } else {
            // Обновляем статус для повторной попытки
            self.job_repository.update(&job).await?;
        }
        
        Ok(())
    }

    /// Повторить выполнение проваленной задачи
    pub async fn retry_failed_job(&self, failed_job_id: i32) -> Result<Job> {
        let failed_job = self.failed_job_repository.find_by_id(failed_job_id).await?
            .ok_or_else(|| anyhow::anyhow!("Failed job not found"))?;

        if !failed_job.can_retry() {
            return Err(anyhow::anyhow!("Job cannot be retried - max attempts exceeded"));
        }

        info!("Retrying failed job ID: {}", failed_job_id);

        // Преобразуем обратно в Job
        let job = failed_job.to_job();
        let created_job = self.job_repository.create(&job).await?;

        // Удаляем из failed_jobs
        self.failed_job_repository.delete(failed_job_id).await?;

        Ok(created_job)
    }

    /// Получить список задач в очереди
    pub async fn list_jobs(&self, queue_name: &QueueName, status: Option<JobStatus>) -> Result<Vec<Job>> {
        match status {
            Some(status) => {
                self.job_repository.find_by_queue_and_status(queue_name, &status).await
            }
            None => {
                // Получаем все задачи из очереди
                let mut all_jobs = Vec::new();
                for status in [JobStatus::Pending, JobStatus::Running, JobStatus::Failed, JobStatus::Completed] {
                    let jobs = self.job_repository.find_by_queue_and_status(queue_name, &status).await?;
                    all_jobs.extend(jobs);
                }
                Ok(all_jobs)
            }
        }
    }

    /// Получить статистику по очередям
    pub async fn get_queue_statistics(&self) -> Result<Vec<QueueStats>> {
        self.job_repository.get_queue_stats().await
    }

    /// Очистить завершенные задачи
    pub async fn cleanup_completed_jobs(&self, older_than_hours: u64) -> Result<i64> {
        let cutoff_time = Utc::now() - chrono::Duration::hours(older_than_hours as i64);
        let deleted_count = self.job_repository.cleanup_completed_jobs(cutoff_time).await?;
        
        info!("Cleaned up {} completed jobs older than {} hours", deleted_count, older_than_hours);
        Ok(deleted_count)
    }

    /// Очистить конкретную очередь
    pub async fn clear_queue(&self, queue_name: &QueueName) -> Result<i64> {
        let jobs = self.list_jobs(queue_name, None).await?;
        let mut deleted_count = 0;

        for job in jobs {
            if let Some(id) = job.id {
                if self.job_repository.delete(id).await? {
                    deleted_count += 1;
                }
            }
        }

        info!("Cleared {} jobs from queue '{}'", deleted_count, queue_name);
        Ok(deleted_count)
    }

    /// Получить информацию о задаче по ID
    pub async fn get_job_info(&self, id: i32) -> Result<Option<Job>> {
        self.job_repository.find_by_id(id).await
    }
}