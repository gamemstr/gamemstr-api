CREATE TABLE locations (
    id              VARCHAR PRIMARY KEY,
    name            VARCHAR NOT NULL,
    description     VARCHAR NOT NULL,
    world_id        VARCHAR NOT NULL,
    map_coordinates JSONB NOT NULL,
    npcs            JSONB NOT NULL
);