// src/domain/repository/job_repository_trait.rs
use async_trait::async_trait;
use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::modules::queue::storage::models::job::Job;

#[async_trait]
pub trait JobRepositoryTrait {
    /// Создать новое задание
    async fn create(&self, job: &Job) -> Result<Job>;
    
    /// Найти задание по ID
    async fn find_by_id(&self, id: i32) -> Result<Option<Job>>;
    
    /// Обновить задание
    async fn update(&self, job: &Job) -> Result<Job>;
    
    /// Удалить задание
    async fn delete(&self, id: i32) -> Result<bool>;
    
    /// Получить следующее готовое к выполнению задание из очереди
    async fn get_next_pending_job(&self, queue_name: &str) -> Result<Option<Job>>;
    
    /// Получить все задания из очереди с определенным статусом
    async fn find_by_queue_and_status(&self, queue_name: &str, status: &str) -> Result<Vec<Job>>;
    
    /// Получить все готовые к выполнению задания (scheduled_at <= now)
    async fn get_ready_jobs(&self, queue_name: &str, limit: i32) -> Result<Vec<Job>>;
    
    /// Получить количество заданий по статусу
    async fn count_by_status(&self, queue_name: &str, status: &str) -> Result<i64>;
    
    /// Отметить задание как запущенное
    async fn mark_as_running(&self, id: i32) -> Result<bool>;
    
    /// Отметить задание как завершенное
    async fn mark_as_completed(&self, id: i32) -> Result<bool>;
    
    /// Отметить задание как проваленное
    async fn mark_as_failed(&self, id: i32) -> Result<bool>;
    
    /// Получить задания для повтора (failed с attempts < max_attempts)
    async fn get_retry_jobs(&self, queue_name: &str) -> Result<Vec<Job>>;
    
    /// Очистить старые завершенные задания
    async fn cleanup_completed_jobs(&self, older_than: DateTime<Utc>) -> Result<i64>;
}