CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE instruments (
    id SERIAL PRIMARY KEY,
    symbol TEXT NOT NULL UNIQUE,
    margin_asset TEXT NOT NULL,
    underlying_asset TEXT NOT NULL,
    maker_fee real NOT NULL,
    taker_fee real NOT NULL,
    routing_fee real NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

