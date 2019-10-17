-- Your SQL goes here

CREATE TABLE "user" (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id VARCHAR NOT NULL UNIQUE,
    user_name VARCHAR NOT NULL UNIQUE
);

CREATE TABLE "post" (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    date VARCHAR NOT NULL DEFAULT now(),
    FOREIGN KEY (user_id) REFERENCES "user" (id)
);