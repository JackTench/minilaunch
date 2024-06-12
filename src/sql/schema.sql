CREATE TABLE IF NOT EXISTS games (
	id INTEGER PRIMARY KEY,
	name TEXT NOT NULL,
	platform TEXT NOT NULL,
	launch_cmd TEXT NOT NULL,
	play_count INTEGER NOT NULL,
	CONSTRAINT unique_game UNIQUE (name, platform)
);
