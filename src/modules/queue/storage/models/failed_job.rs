use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use job::Job;
use job::JobStatus;
use crate::modules::queue::storage::models::job;

/// Структура для представления проваленного задания
/// Соответствует таблице failed_jobs в базе данных
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FailedJob {
    /// Уникальный идентификатор проваленного задания
    pub id: i32,

    /// Имя очереди
    pub queue_name: String,

    /// JSON с данными задачи
    pub payload: String,

    /// Статус выполнения
    pub status: String,

    /// Количество попыток выполнения
    pub attempts: i32,

    /// Максимальное количество попыток
    pub max_attempts: i32,

    /// Сообщение об ошибке
    pub error_message: Option<String>,

    /// Время создания оригинального задания
    pub created_at: DateTime<Utc>,

    /// Время запланированного выполнения
    pub scheduled_at: Option<DateTime<Utc>>,

    /// Время начала выполнения
    pub started_at: Option<DateTime<Utc>>,

    /// Время завершения выполнения
    pub finished_at: Option<DateTime<Utc>>,

    /// Время провала задания
    pub failed_at: DateTime<Utc>,
}

impl FailedJob {
    /// Создать проваленное задание из обычного задания
    pub fn from_job(job: &Job, error_message: String) -> Self {
        Self {
            id: 0, // Будет установлено базой данных
            queue_name: job.queue_name.clone(),
            payload: job.payload.clone(),
            status: JobStatus::Failed.as_str().to_string(),
            attempts: job.attempts,
            max_attempts: job.max_attempts,
            error_message: Some(error_message),
            created_at: job.created_at,
            scheduled_at: job.scheduled_at,
            started_at: job.started_at,
            finished_at: job.finished_at,
            failed_at: Utc::now(),
        }
    }

    /// Получить статус как enum
    pub fn get_status(&self) -> JobStatus {
        JobStatus::from_str(&self.status)
    }

    /// Преобразовать обратно в обычное задание для повторной попытки
    pub fn to_job(&self) -> Job {
        Job {
            id: 0, // Новый ID будет назначен
            queue_name: self.queue_name.clone(),
            payload: self.payload.clone(),
            status: JobStatus::Pending.as_str().to_string(),
            attempts: 0, // Сбрасываем попытки
            max_attempts: self.max_attempts,
            created_at: Utc::now(), // Новое время создания
            scheduled_at: None, // Выполнить сразу
            started_at: None,
            finished_at: None,
        }
    }

    /// Получить сообщение об ошибке или значение по умолчанию
    pub fn get_error_message(&self) -> String {
        self.error_message
            .clone()
            .unwrap_or_else(|| "Unknown error".to_string())
    }

    /// Проверить, было ли задание выполнено хотя бы один раз
    pub fn was_executed(&self) -> bool {
        self.started_at.is_some()
    }

    /// Получить продолжительность выполнения (если есть)
    pub fn execution_duration(&self) -> Option<chrono::Duration> {
        if let (Some(start), Some(finish)) = (&self.started_at, &self.finished_at) {
            Some(*finish - *start)
        } else {
            None
        }
    }
}