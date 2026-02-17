use crate::state::AppState;
use anyhow::Result;
use chrono::Utc;
use clap::{Parser, Subcommand};
use colored::Colorize;
use crate::queues::application::queue_service::JobService;
use crate::queues::domain::entities::job::{Job, JobStatus, FailedJob};
use crate::queues::domain::value_objects::queue_name::QueueName;

#[derive(Parser, Debug)]
pub(crate) struct QueueListCommand {
}

impl QueueListCommand {
    pub async fn execute(&self, state: AppState) -> anyhow::Result<()> {
        println!("{}", "📋 Last 10 Jobs".bright_blue().bold());
        println!();

        // Get all queue statistics
        let queue_stats = state.job_service.get_queue_statistics().await?;

        // If no queues found, display a message
        if queue_stats.is_empty() {
            println!("No queues found. Create a job first.");
        } else {
            // Display jobs for each queue
            for stat in queue_stats {
                let queue_name = stat.queue_name;
                println!("{} {}", "Queue:".bright_yellow(), queue_name.to_string().bright_yellow().bold());

                // Get jobs for this queue
                let jobs = state.job_service.list_jobs(&queue_name, None).await?;

                // Display jobs
                if jobs.is_empty() {
                    println!("  No jobs in this queue");
                } else {
                    for job in jobs.iter().take(10) {
                        self.display_job(job);
                    }
                }

                println!();
            }

            // Display failed jobs section
            println!("{}", "❌ Failed Jobs".bright_red().bold());
            println!();

            // Since we're using a stub repository that always returns empty list,
            // we'll just display a message for now
            println!("No failed jobs found");
        }

        println!("👉 Queue List");
        Ok(())
    }

    fn display_job(&self, job: &Job) {
        let status_str = match job.status {
            JobStatus::Pending => "⏳ Pending".yellow(),
            JobStatus::Running => "▶️ Running".blue(),
            JobStatus::Completed => "✅ Completed".green(),
            JobStatus::Failed => "❌ Failed".red(),
        };

        let id = job.id.map_or("N/A".to_string(), |id| id.to_string());
        let created_at = job.created_at.format("%Y-%m-%d %H:%M:%S").to_string();

        println!("  ID: {} | Status: {} | Created: {} | Attempts: {}/{}",
            id.bright_white(),
            status_str,
            created_at,
            job.attempts,
            job.max_attempts
        );

        // Display additional timing information if available
        if let Some(scheduled_at) = job.scheduled_at {
            println!("    Scheduled: {}", scheduled_at.format("%Y-%m-%d %H:%M:%S"));
        }

        if let Some(started_at) = job.started_at {
            println!("    Started: {}", started_at.format("%Y-%m-%d %H:%M:%S"));
        }

        if let Some(finished_at) = job.finished_at {
            println!("    Finished: {}", finished_at.format("%Y-%m-%d %H:%M:%S"));
        }
    }

    /// Display a failed job with its details
    fn display_failed_job(&self, failed_job: &FailedJob) {
        let id = failed_job.id.map_or("N/A".to_string(), |id| id.to_string());
        let created_at = failed_job.created_at.format("%Y-%m-%d %H:%M:%S").to_string();
        let failed_at = failed_job.failed_at.format("%Y-%m-%d %H:%M:%S").to_string();

        println!("  ID: {} | Failed: {} | Created: {} | Attempts: {}/{}",
            id.bright_white(),
            failed_at.bright_red(),
            created_at,
            failed_job.attempts,
            failed_job.max_attempts
        );

        // Display error message
        println!("    Error: {}", failed_job.error_message.bright_red());

        // Display additional timing information if available
        if let Some(scheduled_at) = failed_job.scheduled_at {
            println!("    Scheduled: {}", scheduled_at.format("%Y-%m-%d %H:%M:%S"));
        }

        if let Some(started_at) = failed_job.started_at {
            println!("    Started: {}", started_at.format("%Y-%m-%d %H:%M:%S"));
        }
    }
}
