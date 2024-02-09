CREATE TABLE sessions (
  id          VARCHAR PRIMARY KEY,
  name        VARCHAR NOT NULL,
  campaign_id VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  notes JSONB NOT NULL,
  plan JSONB  NOT NULL,
  recap JSONB NOT NULL
);
