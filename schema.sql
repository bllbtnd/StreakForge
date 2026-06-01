CREATE TABLE IF NOT EXISTS users (
    username TEXT PRIMARY KEY,
    streak INTEGER NOT NULL DEFAULT 0,
    longest_streak INTEGER NOT NULL DEFAULT 0,
    last_loaded TEXT NOT NULL
);
