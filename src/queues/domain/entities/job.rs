use chrono::{DateTime, Utc};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::queues::domain::value_objects::queue_name::QueueName;

/// Доменная сущность Job - представляет задачу в очереди
#[derive(Debug, Clone, PartialEq)]
pub struct Job {
    pub id: Option<i32>,
    pub queue_name: QueueName,
    pub payload: String,
    pub status: JobStatus,
    pub attempts: i32,
    pub max_attempts: i32,
    pub created_at: DateTime<Utc>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Running,
    Failed,
    Completed,
}

impl Job {
    /// Создать новую задачу
    pub fn new(queue_name: QueueName, payload: String) -> Self {
        Self {
            id: None,
            queue_name,
            payload,
            status: JobStatus::Pending,
            attempts: 0,
            max_attempts: 3,
            created_at: Utc::now(),
            scheduled_at: None,
            started_at: None,
            finished_at: None,
        }
    }

    /// Создать отложенную задачу
    pub fn new_delayed(queue_name: QueueName, payload: String, scheduled_at: DateTime<Utc>) -> Self {
        let mut job = Self::new(queue_name, payload);
        job.scheduled_at = Some(scheduled_at);
        job
    }

    /// Проверить, готова ли задача к выполнению
    pub fn is_ready_to_execute(&self) -> bool {
        (match &self.scheduled_at {
            Some(scheduled) => *scheduled <= Utc::now(),
            None => true,
        }) && self.status == JobStatus::Pending
    }

    /// Начать выполнение задачи
    pub fn start_execution(&mut self) -> Result<()> {
        if self.status != JobStatus::Pending {
            return Err(anyhow::anyhow!("Job is not in pending status"));
        }
        
        self.status = JobStatus::Running;
        self.started_at = Some(Utc::now());
        self.attempts += 1;
        
        Ok(())
    }

    /// Пометить задачу как выполненную
    pub fn mark_completed(&mut self) {
        self.status = JobStatus::Completed;
        self.finished_at = Some(Utc::now());
    }

    /// Пометить задачу как проваленную
    pub fn mark_failed(&mut self) {
        self.status = JobStatus::Failed;
        self.finished_at = Some(Utc::now());
    }

    /// Проверить, превышено ли максимальное количество попыток
    pub fn is_max_attempts_exceeded(&self) -> bool {
        self.attempts >= self.max_attempts
    }

    /// Можно ли повторить выполнение задачи
    pub fn can_retry(&self) -> bool {
        self.status == JobStatus::Failed && !self.is_max_attempts_exceeded()
    }

    /// Сбросить для повторного выполнения
    pub fn reset_for_retry(&mut self) {
        self.status = JobStatus::Pending;
        self.started_at = None;
        self.finished_at = None;
    }
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::Pending => write!(f, "pending"),
            JobStatus::Running => write!(f, "running"),
            JobStatus::Failed => write!(f, "failed"),
            JobStatus::Completed => write!(f, "completed"),
        }
    }
}

impl std::str::FromStr for JobStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(JobStatus::Pending),
            "running" => Ok(JobStatus::Running),
            "failed" => Ok(JobStatus::Failed),
            "completed" => Ok(JobStatus::Completed),
            _ => Err(anyhow::anyhow!("Unknown job status: {}", s)),
        }
    }
}