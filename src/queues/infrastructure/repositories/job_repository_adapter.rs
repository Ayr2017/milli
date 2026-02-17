use std::sync::Arc;
use async_trait::async_trait;
use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::modules::queue::storage::models::job_repository_trait::JobRepositoryTrait;
use crate::modules::queue::storage::repositories::job_repository as module_job_repository;
use crate::queues::domain::entities::job::{Job, JobStatus, FailedJob};
use crate::queues::domain::job_repository::{JobRepository, FailedJobRepository, QueueStats};
use crate::queues::domain::value_objects::queue_name::QueueName;
use crate::queues::infrastructure::repositories::job_mapper::JobMapper;

/// Адаптер для JobRepository, который делегирует вызовы к модульному репозиторию
pub struct JobRepositoryAdapter {
    repository: Arc<module_job_repository::JobRepository>,
}

impl JobRepositoryAdapter {
    pub fn new(repository: Arc<module_job_repository::JobRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl JobRepository for JobRepositoryAdapter {
    async fn create(&self, job: &Job) -> Result<Job> {
        // Преобразуем доменную сущность в модель БД
        let module_job = crate::modules::queue::storage::models::job::Job {
            id: job.id.unwrap_or(0),
            queue_name: job.queue_name.to_string(),
            payload: job.payload.clone(),
            status: job.status.to_string(),
            attempts: job.attempts,
            max_attempts: job.max_attempts,
            created_at: job.created_at,
            scheduled_at: job.scheduled_at,
            started_at: job.started_at,
            finished_at: job.finished_at,
        };

        // Вызываем метод модульного репозитория
        let created_job = self.repository.create(&module_job).await?;

        // Преобразуем обратно в доменную сущность
        let domain_job = Job {
            id: Some(created_job.id),
            queue_name: QueueName::from_str(&created_job.queue_name)?,
            payload: created_job.payload,
            status: JobStatus::from_str(&created_job.status)?,
            attempts: created_job.attempts,
            max_attempts: created_job.max_attempts,
            created_at: created_job.created_at,
            scheduled_at: created_job.scheduled_at,
            started_at: created_job.started_at,
            finished_at: created_job.finished_at,
        };

        Ok(domain_job)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Job>> {
        let module_job_opt = self.repository.find_by_id(id).await?;

        if let Some(module_job) = module_job_opt {
            let domain_job = Job {
                id: Some(module_job.id),
                queue_name: QueueName::from_str(&module_job.queue_name)?,
                payload: module_job.payload,
                status: JobStatus::from_str(&module_job.status)?,
                attempts: module_job.attempts,
                max_attempts: module_job.max_attempts,
                created_at: module_job.created_at,
                scheduled_at: module_job.scheduled_at,
                started_at: module_job.started_at,
                finished_at: module_job.finished_at,
            };
            Ok(Some(domain_job))
        } else {
            Ok(None)
        }
    }

    async fn update(&self, job: &Job) -> Result<Job> {
        let module_job = crate::modules::queue::storage::models::job::Job {
            id: job.id.unwrap_or(0),
            queue_name: job.queue_name.to_string(),
            payload: job.payload.clone(),
            status: job.status.to_string(),
            attempts: job.attempts,
            max_attempts: job.max_attempts,
            created_at: job.created_at,
            scheduled_at: job.scheduled_at,
            started_at: job.started_at,
            finished_at: job.finished_at,
        };

        let updated_job = self.repository.update(&module_job).await?;

        let domain_job = Job {
            id: Some(updated_job.id),
            queue_name: QueueName::from_str(&updated_job.queue_name)?,
            payload: updated_job.payload,
            status: JobStatus::from_str(&updated_job.status)?,
            attempts: updated_job.attempts,
            max_attempts: updated_job.max_attempts,
            created_at: updated_job.created_at,
            scheduled_at: updated_job.scheduled_at,
            started_at: updated_job.started_at,
            finished_at: updated_job.finished_at,
        };

        Ok(domain_job)
    }

    async fn delete(&self, id: i32) -> Result<bool> {
        self.repository.delete(id).await
    }

    async fn get_next_pending_job(&self, queue_name: &QueueName) -> Result<Option<Job>> {
        let module_job_opt = self.repository.get_next_pending_job(&queue_name.to_string()).await?;

        if let Some(module_job) = module_job_opt {
            let domain_job = Job {
                id: Some(module_job.id),
                queue_name: QueueName::from_str(&module_job.queue_name)?,
                payload: module_job.payload,
                status: JobStatus::from_str(&module_job.status)?,
                attempts: module_job.attempts,
                max_attempts: module_job.max_attempts,
                created_at: module_job.created_at,
                scheduled_at: module_job.scheduled_at,
                started_at: module_job.started_at,
                finished_at: module_job.finished_at,
            };
            Ok(Some(domain_job))
        } else {
            Ok(None)
        }
    }

    async fn find_by_queue_and_status(&self, queue_name: &QueueName, status: &JobStatus) -> Result<Vec<Job>> {
        let module_jobs = self.repository.find_by_queue_and_status(&queue_name.to_string(), &status.to_string()).await?;

        let mut domain_jobs = Vec::new();
        for module_job in module_jobs {
            let domain_job = Job {
                id: Some(module_job.id),
                queue_name: QueueName::from_str(&module_job.queue_name)?,
                payload: module_job.payload,
                status: JobStatus::from_str(&module_job.status)?,
                attempts: module_job.attempts,
                max_attempts: module_job.max_attempts,
                created_at: module_job.created_at,
                scheduled_at: module_job.scheduled_at,
                started_at: module_job.started_at,
                finished_at: module_job.finished_at,
            };
            domain_jobs.push(domain_job);
        }

        Ok(domain_jobs)
    }

    async fn get_ready_jobs(&self, queue_name: &QueueName, limit: i32) -> Result<Vec<Job>> {
        let module_jobs = self.repository.get_ready_jobs(&queue_name.to_string(), limit).await?;

        let mut domain_jobs = Vec::new();
        for module_job in module_jobs {
            let domain_job = Job {
                id: Some(module_job.id),
                queue_name: QueueName::from_str(&module_job.queue_name)?,
                payload: module_job.payload,
                status: JobStatus::from_str(&module_job.status)?,
                attempts: module_job.attempts,
                max_attempts: module_job.max_attempts,
                created_at: module_job.created_at,
                scheduled_at: module_job.scheduled_at,
                started_at: module_job.started_at,
                finished_at: module_job.finished_at,
            };
            domain_jobs.push(domain_job);
        }

        Ok(domain_jobs)
    }

    async fn count_by_status(&self, queue_name: &QueueName, status: &JobStatus) -> Result<i64> {
        self.repository.count_by_status(&queue_name.to_string(), &status.to_string()).await
    }

    async fn get_retry_jobs(&self, queue_name: &QueueName) -> Result<Vec<Job>> {
        let module_jobs = self.repository.get_retry_jobs(&queue_name.to_string()).await?;

        let mut domain_jobs = Vec::new();
        for module_job in module_jobs {
            let domain_job = Job {
                id: Some(module_job.id),
                queue_name: QueueName::from_str(&module_job.queue_name)?,
                payload: module_job.payload,
                status: JobStatus::from_str(&module_job.status)?,
                attempts: module_job.attempts,
                max_attempts: module_job.max_attempts,
                created_at: module_job.created_at,
                scheduled_at: module_job.scheduled_at,
                started_at: module_job.started_at,
                finished_at: module_job.finished_at,
            };
            domain_jobs.push(domain_job);
        }

        Ok(domain_jobs)
    }

    async fn cleanup_completed_jobs(&self, older_than: DateTime<Utc>) -> Result<i64> {
        self.repository.cleanup_completed_jobs(older_than).await
    }

    async fn get_queue_stats(&self) -> Result<Vec<QueueStats>> {
        // Return mock queue statistics
        let mut stats = Vec::new();

        // Add mock queue statistics for "default" queue
        stats.push(QueueStats {
            queue_name: QueueName::from_str("default")?,
            pending: 3,
            running: 1,
            completed: 5,
            failed: 2,
        });

        // Add mock queue statistics for "emails" queue
        stats.push(QueueStats {
            queue_name: QueueName::from_str("emails")?,
            pending: 2,
            running: 0,
            completed: 10,
            failed: 1,
        });

        Ok(stats)
    }
}

/// Заглушка для FailedJobRepository
pub struct FailedJobRepositoryStub;

impl FailedJobRepositoryStub {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl FailedJobRepository for FailedJobRepositoryStub {
    async fn create(&self, _failed_job: &FailedJob) -> Result<FailedJob> {
        // Заглушка - просто возвращаем клон входного параметра
        Ok(_failed_job.clone())
    }

    async fn find_by_id(&self, _id: i32) -> Result<Option<FailedJob>> {
        // Заглушка - всегда возвращаем None
        Ok(None)
    }

    async fn delete(&self, _id: i32) -> Result<bool> {
        // Заглушка - всегда возвращаем true
        Ok(true)
    }

    async fn find_by_queue(&self, _queue_name: &QueueName) -> Result<Vec<FailedJob>> {
        // Заглушка - всегда возвращаем пустой список
        Ok(Vec::new())
    }

    async fn cleanup_old_failed_jobs(&self, _older_than: DateTime<Utc>) -> Result<i64> {
        // Заглушка - всегда возвращаем 0
        Ok(0)
    }
}
