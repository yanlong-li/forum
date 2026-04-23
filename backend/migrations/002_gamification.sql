-- User stats for points and level
ALTER TABLE users ADD COLUMN points INTEGER DEFAULT 0;
ALTER TABLE users ADD COLUMN level INTEGER DEFAULT 1;

-- Signups
CREATE TABLE IF NOT EXISTS user_signups (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    signup_date TEXT NOT NULL,
    consecutive_days INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    UNIQUE(user_id, signup_date)
);

-- Badges
CREATE TABLE IF NOT EXISTS badges (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    icon TEXT,
    criteria TEXT,
    created_at TEXT NOT NULL
);

-- User badges
CREATE TABLE IF NOT EXISTS user_badges (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    badge_id TEXT NOT NULL REFERENCES badges(id),
    earned_at TEXT NOT NULL,
    UNIQUE(user_id, badge_id)
);

-- Post view count
ALTER TABLE posts ADD COLUMN view_count INTEGER DEFAULT 0;

-- Featured/Pinned posts
ALTER TABLE posts ADD COLUMN is_pinned INTEGER DEFAULT 0;
ALTER TABLE posts ADD COLUMN is_featured INTEGER DEFAULT 0;

-- Comment likes
CREATE TABLE IF NOT EXISTS comment_votes (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    comment_id TEXT NOT NULL REFERENCES comments(id),
    value INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    UNIQUE(user_id, comment_id)
);

-- User blocks
CREATE TABLE IF NOT EXISTS user_blocks (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    blocked_user_id TEXT NOT NULL REFERENCES users(id),
    created_at TEXT NOT NULL,
    UNIQUE(user_id, blocked_user_id)
);

-- Drafts
CREATE TABLE IF NOT EXISTS drafts (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    draft_type TEXT NOT NULL,
    title TEXT,
    content TEXT,
    updated_at TEXT NOT NULL
);

-- Folders for bookmarks
CREATE TABLE IF NOT EXISTS folders (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    name TEXT NOT NULL,
    created_at TEXT NOT NULL
);

ALTER TABLE bookmarks ADD COLUMN folder_id TEXT REFERENCES folders(id);

-- Best answer
ALTER TABLE comments ADD COLUMN is_accepted INTEGER DEFAULT 0;

-- Announcements
CREATE TABLE IF NOT EXISTS announcements (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL
);

-- Insert default badges
INSERT INTO badges (id, name, description, icon, criteria, created_at) VALUES
    ('badge_first_post', 'First Post', 'Created your first post', '📝', '{"type":"post_count","value":1}', datetime('now')),
    ('badge_posts_10', 'Prolific Writer', 'Created 10 posts', '✍️', '{"type":"post_count","value":10}', datetime('now')),
    ('badge_posts_50', 'Veteran', 'Created 50 posts', '🏆', '{"type":"post_count","value":50}', datetime('now')),
    ('badge_first_comment', 'Voice Heard', 'Posted your first comment', '💬', '{"type":"comment_count","value":1}', datetime('now')),
    ('badge_comments_50', 'Commentator', 'Posted 50 comments', '🗣️', '{"type":"comment_count","value":50}', datetime('now')),
    ('badge_followers_10', 'Influencer', 'Gained 10 followers', '⭐', '{"type":"follower_count","value":10}', datetime('now')),
    ('badge_followers_100', 'Star', 'Gained 100 followers', '🌟', '{"type":"follower_count","value":100}', datetime('now')),
    ('badge_likes_100', 'Popular', 'Received 100 likes on posts', '❤️', '{"type":"likes_received","value":100}', datetime('now')),
    ('badge_member_1y', 'Veteran Member', 'Member for 1 year', '🎂', '{"type":"member_days","value":365}', datetime('now')),
    ('badge_signin_7', 'Week Warrior', 'Signed in for 7 consecutive days', '🔥', '{"type":"consecutive_signin","value":7}', datetime('now')),
    ('badge_signin_30', 'Month Master', 'Signed in for 30 consecutive days', '💯', '{"type":"consecutive_signin","value":30}', datetime('now'));

-- Indexes
CREATE INDEX IF NOT EXISTS idx_user_signups_user ON user_signups(user_id);
CREATE INDEX IF NOT EXISTS idx_user_signups_date ON user_signups(signup_date);
CREATE INDEX IF NOT EXISTS idx_user_badges_user ON user_badges(user_id);
CREATE INDEX IF NOT EXISTS idx_badges_id ON badges(id);
CREATE INDEX IF NOT EXISTS idx_comment_votes_comment ON comment_votes(comment_id);
CREATE INDEX IF NOT EXISTS idx_user_blocks_user ON user_blocks(user_id);
CREATE INDEX IF NOT EXISTS idx_drafts_user ON drafts(user_id);
CREATE INDEX IF NOT EXISTS idx_folders_user ON folders(user_id);
CREATE INDEX IF NOT EXISTS idx_posts_view_count ON posts(view_count DESC);
CREATE INDEX IF NOT EXISTS idx_announcements_active ON announcements(is_active);
