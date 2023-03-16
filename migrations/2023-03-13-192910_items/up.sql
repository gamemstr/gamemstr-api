CREATE TABLE items
(
    id          VARCHAR PRIMARY KEY,
    name        VARCHAR NOT NULL,
    attributes  JSONB NOT NULL
);
