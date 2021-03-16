-- Your SQL goes here
CREATE TABLE messages (
    id BIGSERIAL PRIMARY KEY,
    author VARCHAR NOT NULL,
    author_id VARCHAR NOT NULL,
    content VARCHAR NOT NULL,
    attachment VARCHAR,
    channel VARCHAR NOT NULL,
    channel_id VARCHAR NOT NULL,
    time_posted TIMESTAMP,
    mentions VARCHAR,
    reactions VARCHAR
)
