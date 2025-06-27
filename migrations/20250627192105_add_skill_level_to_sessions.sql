-- Add skill_level column to sessions table
ALTER TABLE sessions ADD COLUMN skill_level skill_level;

-- Add constraint to ensure non-Mixed sessions have a skill level
ALTER TABLE sessions ADD CONSTRAINT check_skill_level_required 
    CHECK (
        (session_type = 'X' AND skill_level IS NULL) OR
        (session_type != 'X' AND skill_level IS NOT NULL)
    );

-- Create index on skill_level for better query performance
CREATE INDEX idx_sessions_skill_level ON sessions(skill_level);