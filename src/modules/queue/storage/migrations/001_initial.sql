-- migrations/001_initial.sql
-- Создание таблицы для активных заданий
CREATE TABLE IF NOT EXISTS jobs (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,                 -- ID задания
    queue_name TEXT NOT NULL DEFAULT 'default',   -- Имя очереди (как в Laravel)
    payload TEXT NOT NULL,                        -- JSON с данными задачи
    status TEXT NOT NULL DEFAULT 'pending',       -- pending, running, failed
    attempts INTEGER NOT NULL DEFAULT 0,          -- Количество попыток
    max_attempts INTEGER NOT NULL DEFAULT 3,      -- Максимальное число попыток
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    scheduled_at DATETIME,                        -- Для отложенного выполнения (NULL = сразу)
    started_at DATETIME,                          -- Когда начали выполнять
    finished_at DATETIME                          -- Когда завершили (успешно или нет)
);

-- Создание таблицы для неудачных заданий
CREATE TABLE IF NOT EXISTS failed_jobs (
       id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
       queue_name TEXT NOT NULL,
       payload TEXT NOT NULL,
       status TEXT NOT NULL,
       attempts INTEGER NOT NULL,
       max_attempts INTEGER NOT NULL,
       error_message TEXT,                           -- Сообщение об ошибке
       created_at DATETIME NOT NULL,
       scheduled_at DATETIME,
       started_at DATETIME,
       finished_at DATETIME,
       failed_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP  -- Время провала
);

-- Индексы для быстрого поиска задач
CREATE INDEX IF NOT EXISTS idx_jobs_queue_status ON jobs (queue_name, status);
CREATE INDEX IF NOT EXISTS idx_jobs_scheduled ON jobs (scheduled_at);
CREATE INDEX IF NOT EXISTS idx_failed_jobs_failed_at ON failed_jobs (failed_at);

