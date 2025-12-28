use async_trait::async_trait;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{SqlitePool, query_as, query, Row};
use crate::modules::queue::storage::models::job::Job;
use crate::modules::queue::storage::models::job_repository_trait::JobRepositoryTrait;

pub struct JobRepository {
    pool: SqlitePool,
}

impl JobRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl JobRepositoryTrait for JobRepository {
    async fn create(&self, job: &Job) -> Result<Job> {
        let result = query(
            r#"
            INSERT INTO jobs (queue_name, payload, status, attempts, max_attempts, created_at, scheduled_at, started_at, finished_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#
        )
        .bind(&job.queue_name)
        .bind(&job.payload)
        .bind(&job.status)
        .bind(job.attempts)
        .bind(job.max_attempts)
        .bind(job.created_at.to_rfc3339()) // Преобразуем в строку
        .bind(job.scheduled_at.map(|dt| dt.to_rfc3339())) // Option<String>
        .bind(job.started_at.map(|dt| dt.to_rfc3339()))
        .bind(job.finished_at.map(|dt| dt.to_rfc3339()))
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_rowid() as i32;
        self.find_by_id(id).await?.ok_or_else(|| anyhow::anyhow!("Failed to retrieve created job"))
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Job>> {
        let row = query("SELECT * FROM jobs WHERE id = ?1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let job = Job {
                id: row.get("id"),
                queue_name: row.get("queue_name"),
                payload: row.get("payload"),
                status: row.get("status"),
                attempts: row.get("attempts"),
                max_attempts: row.get("max_attempts"),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
                scheduled_at: row.get::<Option<String>, _>("scheduled_at")
                    .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                    .transpose()?,
                started_at: row.get::<Option<String>, _>("started_at")
                    .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                    .transpose()?,
                finished_at: row.get::<Option<String>, _>("finished_at")
                    .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                    .transpose()?,
            };
            Ok(Some(job))
        } else {
            Ok(None)
        }
    }

    async fn update(&self, job: &Job) -> Result<Job> {
        query(
            r#"
            UPDATE jobs 
            SET queue_name = ?2, payload = ?3, status = ?4, attempts = ?5, max_attempts = ?6,
                created_at = ?7, scheduled_at = ?8, started_at = ?9, finished_at = ?10
            WHERE id = ?1
            "#
        )
        .bind(job.id)
        .bind(&job.queue_name)
        .bind(&job.payload)
        .bind(&job.status)
        .bind(job.attempts)
        .bind(job.max_attempts)
        .bind(job.created_at.to_rfc3339())
        .bind(job.scheduled_at.map(|dt| dt.to_rfc3339()))
        .bind(job.started_at.map(|dt| dt.to_rfc3339()))
        .bind(job.finished_at.map(|dt| dt.to_rfc3339()))
        .execute(&self.pool)
        .await?;

        Ok(job.clone())
    }

    async fn delete(&self, id: i32) -> Result<bool> {
        let result = query("DELETE FROM jobs WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_next_pending_job(&self, queue_name: &str) -> Result<Option<Job>> {
        let row = query(
            r#"
            SELECT * FROM jobs 
            WHERE queue_name = ?1 AND status = 'pending' 
                AND (scheduled_at IS NULL OR scheduled_at <= datetime('now'))
            ORDER BY created_at ASC
            LIMIT 1
            "#
        )
        .bind(queue_name)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let job = Job {
                id: row.get("id"),
                queue_name: row.get("queue_name"),
                payload: row.get("payload"),
                status: row.get("status"),
                attempts: row.get("attempts"),
                max_attempts: row.get("max_attempts"),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
                scheduled_at: row.get::<Option<String>, _>("scheduled_at")
                    .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                    .transpose()?,
                started_at: row.get::<Option<String>, _>("started_at")
                    .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                    .transpose()?,
                finished_at: row.get::<Option<String>, _>("finished_at")
                    .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                    .transpose()?,
            };
            Ok(Some(job))
        } else {
            Ok(None)
        }
    }

    async fn find_by_queue_and_status(&self, queue_name: &str, status: &str) -> Result<Vec<Job>> {
        let rows = query(
            "SELECT * FROM jobs WHERE queue_name = ?1 AND status = ?2 ORDER BY created_at ASC"
        )
        .bind(queue_name)
        .bind(status)
        .fetch_all(&self.pool)
        .await?;

        let mut jobs = Vec::new();
        for row in rows {
            let job = Job {
                id: row.get("id"),
                queue_name: row.get("queue_name"),
                payload: row.get("payload"),
                status: row.get("status"),
                attempts: row.get("attempts"),
                max_attempts: row.get("max_attempts"),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
                scheduled_at: row.get::<Option<String>, _>("scheduled_at")
                    .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                    .transpose()?,
                started_at: row.get::<Option<String>, _>("started_at")
                    .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                    .transpose()?,
                finished_at: row.get::<Option<String>, _>("finished_at")
                    .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                    .transpose()?,
            };
            jobs.push(job);
        }
        Ok(jobs)
    }

    async fn get_ready_jobs(&self, queue_name: &str, limit: i32) -> Result<Vec<Job>> {
        let rows = query(
            r#"
            SELECT * FROM jobs 
            WHERE queue_name = ?1 AND status = 'pending'
                AND (scheduled_at IS NULL OR scheduled_at <= datetime('now'))
            ORDER BY created_at ASC
            LIMIT ?2
            "#
        )
        .bind(queue_name)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut jobs = Vec::new();
        for row in rows {
            let job = Job {
                id: row.get("id"),
                queue_name: row.get("queue_name"),
                payload: row.get("payload"),
                status: row.get("status"),
                attempts: row.get("attempts"),
                max_attempts: row.get("max_attempts"),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
                scheduled_at: row.get::<Option<String>, _>("scheduled_at")
                    .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                    .transpose()?,
                started_at: row.get::<Option<String>, _>("started_at")
                    .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                    .transpose()?,
                finished_at: row.get::<Option<String>, _>("finished_at")
                    .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                    .transpose()?,
            };
            jobs.push(job);
        }
        Ok(jobs)
    }

    async fn count_by_status(&self, queue_name: &str, status: &str) -> Result<i64> {
        let row = query(
            "SELECT COUNT(*) as count FROM jobs WHERE queue_name = ?1 AND status = ?2"
        )
        .bind(queue_name)
        .bind(status)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.get::<i64, _>("count"))
    }

    async fn mark_as_running(&self, id: i32) -> Result<bool> {
        let result = query(
            r#"
            UPDATE jobs 
            SET status = 'running', started_at = ?, attempts = attempts + 1
            WHERE id = ?
            "#
        )
        .bind(Utc::now().to_rfc3339())
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn mark_as_completed(&self, id: i32) -> Result<bool> {
        let result = query(
            r#"
            UPDATE jobs 
            SET status = 'completed', finished_at = ?
            WHERE id = ?
            "#
        )
        .bind(Utc::now().to_rfc3339())
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn mark_as_failed(&self, id: i32) -> Result<bool> {
        let result = query(
            r#"
            UPDATE jobs 
            SET status = 'failed', finished_at = ?
            WHERE id = ?
            "#
        )
        .bind(Utc::now().to_rfc3339())
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_retry_jobs(&self, queue_name: &str) -> Result<Vec<Job>> {
        let rows = query(
            r#"
            SELECT * FROM jobs 
            WHERE queue_name = ?1 AND status = 'failed' AND attempts < max_attempts
            ORDER BY created_at ASC
            "#
        )
        .bind(queue_name)
        .fetch_all(&self.pool)
        .await?;

        let mut jobs = Vec::new();
        for row in rows {
            let job = Job {
                id: row.get("id"),
                queue_name: row.get("queue_name"),
                payload: row.get("payload"),
                status: row.get("status"),
                attempts: row.get("attempts"),
                max_attempts: row.get("max_attempts"),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
                scheduled_at: row.get::<Option<String>, _>("scheduled_at")
                    .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                    .transpose()?,
                started_at: row.get::<Option<String>, _>("started_at")
                    .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                    .transpose()?,
                finished_at: row.get::<Option<String>, _>("finished_at")
                    .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                    .transpose()?,
            };
            jobs.push(job);
        }
        Ok(jobs)
    }

    async fn cleanup_completed_jobs(&self, older_than: DateTime<Utc>) -> Result<i64> {
        let result = query("DELETE FROM jobs WHERE status = 'completed' AND finished_at < ?1")
            .bind(older_than.to_rfc3339())
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() as i64)
    }
}