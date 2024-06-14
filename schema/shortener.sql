CREATE DATABASE shortener;

# table urls
CREATE table IF NOT EXISTS urls (
    id CHAR(6) PRIMARY KEY,
    url TEXT NOT NULL UNIQUE
);
