-- Add up migration script here
CREATE TABLE IF NOT EXISTS note (
    id UUID PRIMARY KEY NOT NULL,
    title VARCHAR(255) NOT NULL UNIQUE,
    author VARCHAR(255) NOT NULL UNIQUE,
    category VARCHAR(100),
    content TEXT NOT NULL,
    published BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);