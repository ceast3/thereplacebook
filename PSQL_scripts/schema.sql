CREATE TABLE users (
                       id INTEGER PRIMARY KEY AUTOINCREMENT,
                       name TEXT NOT NULL,
                       image_url TEXT NOT NULL,
                       rating FLOAT DEFAULT 1200
);

CREATE TABLE matches (
                         id INTEGER PRIMARY KEY AUTOINCREMENT,
                         winner_id INTEGER NOT NULL,
                         loser_id INTEGER NOT NULL,
                         FOREIGN KEY (winner_id) REFERENCES users(id),
                         FOREIGN KEY (loser_id) REFERENCES users(id)
);