CREATE TABLE campaigns (
  id VARCHAR PRIMARY KEY,
  name VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  world_id VARCHAR NOT NULL,
  players JSONB NOT NULL
);
