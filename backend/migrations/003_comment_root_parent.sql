-- Add root_parent_id to comments for efficient nested comment queries
ALTER TABLE comments ADD COLUMN root_parent_id TEXT REFERENCES comments(id);

-- Index for efficient lookup of replies by root parent
CREATE INDEX IF NOT EXISTS idx_comments_root_parent ON comments(root_parent_id);
