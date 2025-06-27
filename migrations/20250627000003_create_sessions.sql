-- Create session type enum
CREATE TYPE session_type AS ENUM ('C', 'S', 'L', 'X');

-- Create sessions table
CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    session_type session_type NOT NULL,
    datetime TIMESTAMPTZ NOT NULL,
    duration_minutes INTEGER NOT NULL CHECK (duration_minutes >= 60 AND duration_minutes <= 120),
    venue_id UUID NOT NULL REFERENCES venues(id),
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    CONSTRAINT duration_30_min_increment CHECK (duration_minutes % 30 = 0)
);

-- Create indexes
CREATE INDEX idx_sessions_datetime ON sessions(datetime);
CREATE INDEX idx_sessions_venue_id ON sessions(venue_id);
CREATE INDEX idx_sessions_type ON sessions(session_type);