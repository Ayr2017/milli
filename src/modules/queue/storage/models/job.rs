use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

/// Структура для представления задания в очереди
/// Соответствует таблице jobs в базе данных
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Job {
    /// Уникальный идентификатор задания
    pub id: i32,

    /// Имя очереди (по умолчанию 'default')
    pub queue_name: String,

    /// JSON с данными задачи
    pub payload: String,

    /// Статус выполнения: pending, running, failed, completed
    pub status: String,

    /// Количество попыток выполнения
    pub attempts: i32,

    /// Максимальное количество попыток
    pub max_attempts: i32,

    /// Время создания задания
    pub created_at: DateTime<Utc>,

    /// Время запланированного выполнения (для отложенных задач)
    pub scheduled_at: Option<DateTime<Utc>>,

    /// Время начала выполнения
    pub started_at: Option<DateTime<Utc>>,

    /// Время завершения выполнения
    pub finished_at: Option<DateTime<Utc>>,
}

/// Статусы выполнения задания
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JobStatus {
    /// Ожидает выполнения
    Pending,
    /// Выполняется
    Running,
    /// Завершено успешно
    Completed,
    /// Завершено с ошибкой
    Failed,
}

impl JobStatus {
    /// Преобразование в строку для базы данных
    pub fn as_str(&self) -> &'static str {
        match self {
            JobStatus::Pending => "pending",
            JobStatus::Running => "running",
            JobStatus::Completed => "completed",
            JobStatus::Failed => "failed",
        }
    }

    /// Создание из строки
    pub fn from_str(s: &str) -> Self {
        match s {
            "pending" => JobStatus::Pending,
            "running" => JobStatus::Running,
            "completed" => JobStatus::Completed,
            "failed" => JobStatus::Failed,
            _ => JobStatus::Pending, // По умолчанию
        }
    }
}

impl Job {
    /// Создать новое задание
    pub fn new(queue_name: String, payload: String) -> Self {
        Self {
            id: 0, // Будет установлено базой данных
            queue_name,
            payload,
            status: JobStatus::Pending.as_str().to_string(),
            attempts: 0,
            max_attempts: 3,
            created_at: Utc::now(),
            scheduled_at: None,
            started_at: None,
            finished_at: None,
        }
    }

    /// Создать отложенное задание
    pub fn new_scheduled(queue_name: String, payload: String, scheduled_at: DateTime<Utc>) -> Self {
        let mut job = Self::new(queue_name, payload);
        job.scheduled_at = Some(scheduled_at);
        job
    }

    /// Проверить, готово ли задание к выполнению
    pub fn is_ready(&self) -> bool {
        match self.scheduled_at {
            Some(scheduled) => Utc::now() >= scheduled,
            None => true,
        }
    }

    /// Проверить, можно ли повторить задание
    pub fn can_retry(&self) -> bool {
        self.attempts < self.max_attempts
    }

    /// Получить статус как enum
    pub fn get_status(&self) -> JobStatus {
        JobStatus::from_str(&self.status)
    }

    /// Установить статус
    pub fn set_status(&mut self, status: JobStatus) {
        self.status = status.as_str().to_string();
    }

    /// Отметить начало выполнения
    pub fn mark_started(&mut self) {
        self.status = JobStatus::Running.as_str().to_string();
        self.started_at = Some(Utc::now());
        self.attempts += 1;
    }

    /// Отметить успешное завершение
    pub fn mark_completed(&mut self) {
        self.status = JobStatus::Completed.as_str().to_string();
        self.finished_at = Some(Utc::now());
    }

    /// Отметить неудачное выполнение
    pub fn mark_failed(&mut self) {
        self.status = JobStatus::Failed.as_str().to_string();
        self.finished_at = Some(Utc::now());
    }
}