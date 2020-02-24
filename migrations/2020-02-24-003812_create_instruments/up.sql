CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE instruments (
id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
index_id UUID NOT NULL REFERENCES index (id),
symbol VARCHAR(254) NOT NULL,
margin_asset VARCHAR(254) NOT NULL,
underlying_asset VARCHAR(254) NOT NULL,
maker_fee FLOAT NOT NULL,
taker_fee FLOAT NOT NULL,
routing_fee FLOAT NOT NULL,
created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

SELECT diesel_manage_updated_at('instruments');
