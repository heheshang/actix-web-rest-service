-- Your SQL goes here
CREATE TABLE login_history (
    id Serial PRIMARY KEY not null,
    user_id INTEGER not null references users(id),
    login_timestamp TIMESTAMP not null
);
