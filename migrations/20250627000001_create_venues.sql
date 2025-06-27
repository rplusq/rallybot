-- Create venues table
CREATE TABLE venues (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    address TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

-- Create index on name for searching
CREATE INDEX idx_venues_name ON venues(name);