-- Your SQL goes here
CREATE TABLE people (
    id SERIAL PRIMARY KEY not null,
    name VARCHAR(255) not null,
    gender BOOLEAN not null,
    age INTEGER not null,
    address VARCHAR(255) not null,
    phone VARCHAR(255) not null,
    email VARCHAR(255) not null,
    created_at TIMESTAMP not null default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP not null default CURRENT_TIMESTAMP
);
