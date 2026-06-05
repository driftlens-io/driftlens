CREATE TABLE users (
    id          VARCHAR NOT NULL PRIMARY KEY,
    username    VARCHAR(100) NOT NULL,
    email       VARCHAR(255) NOT NULL UNIQUE,
    password    VARCHAR NOT NULL,
    first_access BOOLEAN DEFAULT false
);

CREATE TABLE post (
    id          VARCHAR NOT NULL PRIMARY KEY,
    title       VARCHAR(255) NOT NULL,
    description VARCHAR,
    status      SMALLINT NOT NULL,
    start_date  DATE,
    end_date    DATE,
    created_at  TIMESTAMP WITH TIME ZONE,
    updated_at  TIMESTAMP WITH TIME ZONE,
    deleted_at  TIMESTAMP WITH TIME ZONE,
    author_id   VARCHAR NOT NULL REFERENCES users(id)
);

CREATE TABLE comment (
    id          VARCHAR NOT NULL PRIMARY KEY,
    content     VARCHAR NOT NULL,
    created_at  TIMESTAMP WITH TIME ZONE,
    updated_at  TIMESTAMP WITH TIME ZONE,
    post_id     VARCHAR NOT NULL REFERENCES post(id),
    author_id   VARCHAR NOT NULL REFERENCES users(id)
);