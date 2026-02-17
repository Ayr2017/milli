use async_trait::async_trait;
use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::queues::domain::entities::job::{Job, JobStatus, FailedJob};
use crate::queues::domain::value_objects::queue_name::QueueName;

/// Интерфейс репозитория для работы с Job'ами
#[async_trait]
pub trait JobRepository: Send + Sync {
    /// Создать новую задачу
    async fn create(&self, job: &Job) -> Result<Job>;

    /// Найти задачу по ID
    async fn find_by_id(&self, id: i32) -> Result<Option<Job>>;

    /// Обновить задачу
    async fn update(&self, job: &Job) -> Result<Job>;

    /// Удалить задачу
    async fn delete(&self, id: i32) -> Result<bool>;

    /// Получить следующую готовую к выполнению задачу из очереди
    async fn get_next_pending_job(&self, queue_name: &QueueName) -> Result<Option<Job>>;

    /// Найти задачи по очереди и статусу
    async fn find_by_queue_and_status(&self, queue_name: &QueueName, status: &JobStatus) -> Result<Vec<Job>>;

    /// Получить готовые к выполнению задачи
    async fn get_ready_jobs(&self, queue_name: &QueueName, limit: i32) -> Result<Vec<Job>>;

    /// Подсчитать количество задач по статусу
    async fn count_by_status(&self, queue_name: &QueueName, status: &JobStatus) -> Result<i64>;

    /// Получить задачи для повтора (failed + attempts < max_attempts)
    async fn get_retry_jobs(&self, queue_name: &QueueName) -> Result<Vec<Job>>;

    /// Очистить завершенные задачи старше указанной даты
    async fn cleanup_completed_jobs(&self, older_than: DateTime<Utc>) -> Result<i64>;

    /// Получить все очереди с количеством задач
    async fn get_queue_stats(&self) -> Result<Vec<QueueStats>>;
}

/// Статистика по очереди
#[derive(Debug, Clone)]
pub struct QueueStats {
    pub queue_name: QueueName,
    pub pending: i64,
    pub running: i64,
    pub completed: i64,
    pub failed: i64,
}

/// Интерфейс репозитория для работы с проваленными задачами
#[async_trait]
pub trait FailedJobRepository: Send + Sync {
    /// Создать новую проваленную задачу
    async fn create(&self, failed_job: &FailedJob) -> Result<FailedJob>;

    /// Найти проваленную задачу по ID
    async fn find_by_id(&self, id: i32) -> Result<Option<FailedJob>>;

    /// Удалить проваленную задачу
    async fn delete(&self, id: i32) -> Result<bool>;

    /// Найти проваленные задачи по имени очереди
    async fn find_by_queue(&self, queue_name: &QueueName) -> Result<Vec<FailedJob>>;

    /// Очистить проваленные задачи старше указанной даты
    async fn cleanup_old_failed_jobs(&self, older_than: DateTime<Utc>) -> Result<i64>;
}
