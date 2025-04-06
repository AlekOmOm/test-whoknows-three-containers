-- db/init.sql 
-- postgresql database initialization script
DROP TABLE IF EXISTS users;

CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL
);

-- Create a default user, The password is 'password' (MD5 hashed)
INSERT INTO users (username, email, password) 
    VALUES ('admin', 'keamonk1@stud.kea.dk', '5f4dcc3b5aa765d61d8327deb882cf99');

CREATE TABLE IF NOT EXISTS pages (
    title TEXT PRIMARY KEY,
    url TEXT NOT NULL UNIQUE,
    language TEXT NOT NULL CHECK(language IN ('en', 'da')) DEFAULT 'en',
    last_updated TIMESTAMP,
    content TEXT NOT NULL
);

-- Add search functionality data
CREATE TABLE IF NOT EXISTS search_results (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    url TEXT NOT NULL
);

-- Sample search data
INSERT INTO search_results (title, description, url)
VALUES 
    ('Example Result 1', 'This is a sample result description', 'https://example.com'),
    ('Example Result 2', 'Another sample result', 'https://example.org');