CREATE TABLE items
(
    id              VARCHAR PRIMARY KEY,
    name            VARCHAR NOT NULL,
    item_type       JSONB NOT NULL,
    rarity          JSONB NOT NULL,
    attunement      JSONB,
    weapon_type     JSONB,
    armor_type      JSONB,
    conditions      JSONB,
    attached_spell  JSONB,
    has_charges     JSONB,
    inventory       JSONB,
    others          JSONB,
    actions         JSONB
);
