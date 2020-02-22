CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    firstname VARCHAR NULL,
    lastname VARCHAR NULL,
    email TEXT NOT NULL UNIQUE,
    phone VARCHAR NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tfa_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    pgp_pub_key TEXT NULL,
    primary_country VARCHAR NOT NULL,
    primary_geoip_country VARCHAR NOT NULL,
    primary_geoip_region VARCHAR NOT NULL,
    user_type VARCHAR NOT NULL
    can_trade BOOLEAN NOT NULL DEFAULT TRUE,
    can_deposit BOOLEAN NOT NULL DEFAULT TRUE,
    can_withdraw BOOLEAN NOT NULL DEFAULT TRUE,
)

SELECT diesel_manage_updated_at('users');
