CREATE TABLE events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type INTEGER NOT NULL,
    subtype INTEGER NOT NULL,
    description TEXT NOT NULL,
    body TEXT NOT NULL,
    scale INTEGER NOT NULL,
    source TEXT NOT NULL,
    transpired TEXT DEFAULT NULL,
    published_date TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(type) REFERENCES types(id),
    FOREIGN KEY(subtype) REFERENCES subtypes(id)
);

-- CREATE TABLE events (
--   id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
--   event_type INTEGER NOT NULL,
--   FOREIGN KEY(event_type) REFERENCES type(id),
--   subtype_id INTEGER NOT NULL,
--   FOREIGN KEY(subtype_id) REFERENCES subtype(id),
--   title VARCHAR NOT NULL,
--   body TEXT NOT NULL,
--   scale INTEGER NOT NULL,
--   transpired TEXT DEFAULT NULL,
--   published_date TEXT DEFAULT CURRENT_TIMESTAMP
-- )
