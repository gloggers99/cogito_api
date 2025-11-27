DROP TABLE IF EXISTS users;

CREATE TABLE users
(
    user_id         SERIAL PRIMARY KEY NOT NULL,
    user_email      TEXT UNIQUE NOT NULL,
    user_phone      TEXT UNIQUE NOT NULL,
    user_name       TEXT NOT NULL,
    user_pass       TEXT NOT NULL,
    user_last_login TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    login_id        UUID DEFAULT NULL UNIQUE,
    verified        BOOLEAN NOT NULL DEFAULT FALSE,
    admin           BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE conversations (
    conversation_id    SERIAL PRIMARY KEY NOT NULL,
    user_id            INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,

    conversation       JSON NOT NULL,
    conversation_title TEXT NOT NULL DEFAULT 'new conversation',

    created_at         TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- allow indexing by user_id for fetching all user convos.
CREATE INDEX idx_conversations_user_id ON conversations(user_id);

ALTER TABLE users
    OWNER TO postgres;

ALTER TABLE conversations
    OWNER TO postgres;
