-- Create registration status enum
CREATE TYPE registration_status AS ENUM ('Confirmed', 'Substitute');

-- Create registrations table
CREATE TABLE registrations (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    session_id UUID NOT NULL REFERENCES sessions(id),
    status registration_status NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    UNIQUE(user_id, session_id)
);

-- Create indexes
CREATE INDEX idx_registrations_session_id ON registrations(session_id);
CREATE INDEX idx_registrations_user_id ON registrations(user_id);
CREATE INDEX idx_registrations_created_at ON registrations(created_at);