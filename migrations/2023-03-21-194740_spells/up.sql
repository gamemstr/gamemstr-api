CREATE TABLE spells (
    id              VARCHAR PRIMARY KEY,
    name            VARCHAR NOT NULL,
    description     VARCHAR NOT NULL,
    level           JSONB NOT NULL,
    casting_time    JSONB NOT NULL,
    duration        JSONB NOT NULL,
    damage          JSONB,
    range           JSONB NOT NULL,
    area            JSONB,
    damage_type     JSONB,
    components      JSONB NOT NULL,
    attack_bonus    INT,
    save            JSONB
);