use serde::{Deserialize, Serialize};
use std::fmt;

/// Типы очередей для индексирования в Meilisearch
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QueueName {
    /// Очередь для индексирования документов
    IndexDocuments,
    /// Очередь для обновления индексов
    UpdateIndexes,
    /// Очередь для удаления документов из индекса
    DeleteDocuments,
    /// Очередь для настройки индексов (synonyms, stop words, etc.)
    ConfigureIndexes,
    /// Очередь для полной переиндексации
    ReindexAll,
    /// Очередь по умолчанию
    Default,
}

impl QueueName {
    /// Получить строковое представление очереди
    pub fn as_str(&self) -> &'static str {
        match self {
            QueueName::IndexDocuments => "index_documents",
            QueueName::UpdateIndexes => "update_indexes", 
            QueueName::DeleteDocuments => "delete_documents",
            QueueName::ConfigureIndexes => "configure_indexes",
            QueueName::ReindexAll => "reindex_all",
            QueueName::Default => "default",
        }
    }

    /// Получить описание очереди
    pub fn description(&self) -> &'static str {
        match self {
            QueueName::IndexDocuments => "Индексирование новых документов в Meilisearch",
            QueueName::UpdateIndexes => "Обновление существующих индексов",
            QueueName::DeleteDocuments => "Удаление документов из индекса",
            QueueName::ConfigureIndexes => "Настройка конфигурации индексов",
            QueueName::ReindexAll => "Полная переиндексация всех данных",
            QueueName::Default => "Очередь по умолчанию",
        }
    }

    /// Получить приоритет очереди (чем меньше число, тем выше приоритет)
    pub fn priority(&self) -> u8 {
        match self {
            QueueName::ReindexAll => 1,        // Самый высокий приоритет
            QueueName::ConfigureIndexes => 2,
            QueueName::DeleteDocuments => 3,
            QueueName::UpdateIndexes => 4,
            QueueName::IndexDocuments => 5,
            QueueName::Default => 10,          // Самый низкий приоритет
        }
    }

    /// Все доступные очереди
    pub fn all() -> Vec<QueueName> {
        vec![
            QueueName::IndexDocuments,
            QueueName::UpdateIndexes,
            QueueName::DeleteDocuments,
            QueueName::ConfigureIndexes,
            QueueName::ReindexAll,
            QueueName::Default,
        ]
    }

    /// Создать очередь из строки
    pub fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "index_documents" => Ok(QueueName::IndexDocuments),
            "update_indexes" => Ok(QueueName::UpdateIndexes),
            "delete_documents" => Ok(QueueName::DeleteDocuments),
            "configure_indexes" => Ok(QueueName::ConfigureIndexes),
            "reindex_all" => Ok(QueueName::ReindexAll),
            "default" => Ok(QueueName::Default),
            _ => Err(anyhow::anyhow!("Unknown queue name: {}", s)),
        }
    }
}

impl fmt::Display for QueueName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<QueueName> for String {
    fn from(queue: QueueName) -> Self {
        queue.as_str().to_string()
    }
}

impl std::str::FromStr for QueueName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        QueueName::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_name_conversion() {
        let queue = QueueName::IndexDocuments;
        assert_eq!(queue.as_str(), "index_documents");
        assert_eq!(queue.to_string(), "index_documents");
    }

    #[test]
    fn test_queue_from_string() {
        let queue = QueueName::from_str("index_documents").unwrap();
        assert_eq!(queue, QueueName::IndexDocuments);
    }

    #[test]
    fn test_priority_order() {
        assert!(QueueName::ReindexAll.priority() < QueueName::IndexDocuments.priority());
        assert!(QueueName::ConfigureIndexes.priority() < QueueName::Default.priority());
    }
}