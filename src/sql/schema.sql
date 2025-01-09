CREATE TABLE IF NOT EXISTS platforms (
	id INTEGER PRIMARY KEY,
	name TEXT NOT NULL,
	launch_prefix TEXT NOT NULL,
	CONSTRAINT unique_platform UNIQUE (launch_prefix)
);

-- Also create a default Steam platform in the platforms table.
INSERT OR IGNORE INTO platforms (name, launch_prefix) VALUES ("Steam", "steam://run/");

CREATE TABLE IF NOT EXISTS games (
	id INTEGER PRIMARY KEY,
	name TEXT NOT NULL,
	platform_id INTEGER NOT NULL,
	launch_postfix TEXT NOT NULL,
	CONSTRAINT unique_game UNIQUE (platform_id, launch_postfix),
	FOREIGN KEY (platform_id) REFERENCES platforms (id)
);
