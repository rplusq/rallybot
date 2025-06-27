-- Create custom enum types
CREATE TYPE gender AS ENUM ('Male', 'Female');
CREATE TYPE skill_level AS ENUM ('A', 'B', 'C', 'D', 'E', 'F', 'G', 'H');
CREATE TYPE preferred_side AS ENUM ('Right', 'Left', 'Flexible');
CREATE TYPE play_frequency AS ENUM ('NeverPlayed', 'FewTimesMonth', 'OnceWeek', 'SeveralTimesWeek');
CREATE TYPE looking_for AS ENUM ('SocialConnections', 'BusinessOpportunities');

-- Create users table
CREATE TABLE users (
    id UUID PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    phone_number VARCHAR(20) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    city VARCHAR(100) NOT NULL,
    photo_url VARCHAR(255),
    occupation VARCHAR(100) NOT NULL,
    company VARCHAR(100) NOT NULL,
    industry VARCHAR(100) NOT NULL,
    linkedin_url VARCHAR(255) NOT NULL,
    gender gender NOT NULL,
    skill_levels skill_level[] NOT NULL,
    preferred_side preferred_side NOT NULL,
    play_frequency play_frequency NOT NULL,
    looking_for looking_for[] NOT NULL,
    is_approved BOOLEAN DEFAULT FALSE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

-- Create indexes
CREATE INDEX idx_users_phone ON users(phone_number);
CREATE INDEX idx_users_is_approved ON users(is_approved);