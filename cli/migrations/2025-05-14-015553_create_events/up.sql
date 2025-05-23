CREATE TABLE events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type INTEGER NOT NULL,
    description TEXT NOT NULL,
    body TEXT NOT NULL,
    scale INTEGER NOT NULL,
    source TEXT NOT NULL UNIQUE,
    transpired TEXT DEFAULT NULL,
    published_date TEXT DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY(type) REFERENCES types(id)
);
