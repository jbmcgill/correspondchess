CREATE TABLE game (
  id INTEGER PRIMARY KEY NOT NULL,
  created BIGINT NOT NULL,
  white TEXT NOT NULL,
  black TEXT NOT NULL
);

CREATE TABLE moves (
  id INTEGER PRIMARY KEY NOT NULL,
  game_id INTEGER NOT NULL,
  player_move TEXT NOT NULL
);
