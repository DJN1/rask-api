CREATE TABLE tasks (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    title VARCHAR(32) NOT NULL,
    description TEXT NULL,
    user_id VARCHAR(36) NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NULL
)