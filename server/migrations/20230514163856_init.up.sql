-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS messages (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	sender_id INTEGER NOT NULL,
	receiver_id INTEGER NOT NULL,
	message_text TEXT NOT NULL,
	sent_at DATETIME DEFAULT (DATETIME(CURRENT_TIMESTAMP, 'LOCALTIME')),
	FOREIGN KEY (sender_id) REFERENCES users (id),
	FOREIGN KEY (receiver_id) REFERENCES users (id)
);

INSERT INTO users (name) VALUES ('Mayur');
INSERT INTO users (name) VALUES ('Nitesh');

INSERT INTO messages (sender_id, receiver_id, message_text) VALUES (1, 2, 'Good Morning');
INSERT INTO messages (sender_id, receiver_id, message_text) VALUES (2, 1, 'Top of the morning');
INSERT INTO messages (sender_id, receiver_id, message_text) VALUES (1, 2, 'Hell yeah');
INSERT INTO messages (sender_id, receiver_id, message_text) VALUES (2, 1, 'Pattis hava dostas');
