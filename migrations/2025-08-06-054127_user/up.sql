-- Your SQL goes here

CREATE TABLE Users(
    -- id SERIAL PRIMARY KEY,
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username Text NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
