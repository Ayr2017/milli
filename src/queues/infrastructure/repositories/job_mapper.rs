use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::Row;
use crate::queues::domain::entities::job::{Job as DomainJob, JobStatus, FailedJob as DomainFailedJob};
use crate::queues::domain::value_objects::queue_name::QueueName;

/// Маппер для преобразования между доменными сущностями и моделями БД
pub struct JobMapper;

impl JobMapper {
    /// Преобразовать Row из БД в доменную сущность Job
    pub fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<DomainJob> {
        let queue_name_str: String = row.get("queue_name");
        let queue_name = QueueName::from_str(&queue_name_str)?;

        let status_str: String = row.get("status");
        let status = JobStatus::from_str(&status_str)?;

        Ok(DomainJob {
            id: Some(row.get("id")),
            queue_name,
            payload: row.get("payload"),
            status,
            attempts: row.get("attempts"),
            max_attempts: row.get("max_attempts"),
            created_at: Self::parse_datetime(&row.get::<String, _>("created_at"))?,
            scheduled_at: Self::parse_optional_datetime(row.get::<Option<String>, _>("scheduled_at"))?,
            started_at: Self::parse_optional_datetime(row.get::<Option<String>, _>("started_at"))?,
            finished_at: Self::parse_optional_datetime(row.get::<Option<String>, _>("finished_at"))?,
        })
    }

    /// Преобразовать список Row в список доменных сущностей
    pub fn from_rows(rows: Vec<sqlx::sqlite::SqliteRow>) -> Result<Vec<DomainJob>> {
        rows.iter()
            .map(Self::from_row)
            .collect()
    }

    /// Парсинг даты из строки
    fn parse_datetime(date_str: &str) -> Result<DateTime<Utc>> {
        DateTime::parse_from_rfc3339(date_str)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| anyhow::anyhow!("Failed to parse datetime: {}", e))
    }

    /// Парсинг опциональной даты
    fn parse_optional_datetime(date_str: Option<String>) -> Result<Option<DateTime<Utc>>> {
        match date_str {
            Some(s) => Ok(Some(Self::parse_datetime(&s)?)),
            None => Ok(None),
        }
    }

    /// Преобразовать DateTime в строку для БД
    pub fn datetime_to_string(dt: &DateTime<Utc>) -> String {
        dt.to_rfc3339()
    }

    /// Преобразовать опциональную DateTime в строку для БД
    pub fn optional_datetime_to_string(dt: Option<&DateTime<Utc>>) -> Option<String> {
        dt.map(|d| d.to_rfc3339())
    }
}

/// Расширение для JobStatus
impl JobStatus {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(JobStatus::Pending),
            "running" => Ok(JobStatus::Running),
            "failed" => Ok(JobStatus::Failed),
            "completed" => Ok(JobStatus::Completed),
            _ => Err(anyhow::anyhow!("Unknown job status: {}", s)),
        }
    }
}

/// Маппер для преобразования между доменными сущностями FailedJob и моделями БД
pub struct FailedJobMapper;

impl FailedJobMapper {
    /// Преобразовать Row из БД в доменную сущность FailedJob
    pub fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<DomainFailedJob> {
        let queue_name_str: String = row.get("queue_name");
        let queue_name = QueueName::from_str(&queue_name_str)?;

        let status_str: String = row.get("status");
        let status = JobStatus::from_str(&status_str)?;

        Ok(DomainFailedJob {
            id: Some(row.get("id")),
            queue_name,
            payload: row.get("payload"),
            status,
            attempts: row.get("attempts"),
            max_attempts: row.get("max_attempts"),
            error_message: row.get::<Option<String>, _>("error_message").unwrap_or_else(|| "Unknown error".to_string()),
            created_at: JobMapper::parse_datetime(&row.get::<String, _>("created_at"))?,
            scheduled_at: JobMapper::parse_optional_datetime(row.get::<Option<String>, _>("scheduled_at"))?,
            started_at: JobMapper::parse_optional_datetime(row.get::<Option<String>, _>("started_at"))?,
            finished_at: JobMapper::parse_optional_datetime(row.get::<Option<String>, _>("finished_at"))?,
            failed_at: JobMapper::parse_datetime(&row.get::<String, _>("failed_at"))?,
        })
    }

    /// Преобразовать список Row в список доменных сущностей FailedJob
    pub fn from_rows(rows: Vec<sqlx::sqlite::SqliteRow>) -> Result<Vec<DomainFailedJob>> {
        rows.iter()
            .map(Self::from_row)
            .collect()
    }
}
