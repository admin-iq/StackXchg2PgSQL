-- StackExchange Data Schema
-- This schema represents the core components of a StackExchange site (like StackOverflow)
-- with optimizations for Apache Superset analytics and reporting.

-- Users table - stores information about registered users
-- Primary source of user data including reputation, activity dates, and voting history
CREATE TABLE users (
    id INTEGER PRIMARY KEY,                -- Unique identifier for each user
    reputation INTEGER NOT NULL DEFAULT 0, -- User's reputation score based on community contributions
    creation_date TIMESTAMP NOT NULL,      -- When the user account was created
    display_name TEXT,                     -- User's displayed name on the site
    last_access_date TIMESTAMP NOT NULL,   -- Last time the user accessed the site
    website_url TEXT,                      -- User's personal website URL
    location TEXT,                         -- User's geographic location
    about_me TEXT,                         -- User's profile description
    views INTEGER DEFAULT 0,               -- Number of times the user's profile was viewed
    up_votes INTEGER DEFAULT 0,            -- Number of upvotes cast by the user
    down_votes INTEGER DEFAULT 0,          -- Number of downvotes cast by the user
    profile_image_url TEXT,                -- URL to user's profile picture
    email_hash TEXT,                       -- Hashed email for Gravatar or privacy purposes
    account_id INTEGER                     -- ID linking to network-wide account if applicable
);

-- Indexes on users table for common query patterns
CREATE INDEX idx_users_reputation ON users (reputation);          -- For sorting/filtering users by reputation
CREATE INDEX idx_users_creation_date ON users (creation_date);    -- For chronological user analysis
CREATE INDEX idx_users_last_access_date ON users (last_access_date); -- For activity analysis
CREATE INDEX idx_users_up_votes ON users (up_votes);              -- For voting pattern analysis
CREATE INDEX idx_users_down_votes ON users (down_votes);          -- For voting pattern analysis
CREATE INDEX idx_users_account_id ON users (account_id);          -- For cross-site user identity lookup

-- Badges table - stores achievements awarded to users
-- Records user accomplishments and incentives for participation
CREATE TABLE badges (
    id        INTEGER      PRIMARY KEY,
    user_id   INTEGER      NOT NULL,      -- User who earned the badge
    name      TEXT         NOT NULL,      -- Name of the badge (e.g., "Good Answer")
    date      TIMESTAMP    NOT NULL,      -- When the badge was awarded
    class     SMALLINT     NOT NULL,      -- Badge level (1=gold, 2=silver, 3=bronze)
    tag_based BOOLEAN      NOT NULL       -- Whether badge is for tag-specific accomplishments
);

-- Indexes on badges table
CREATE INDEX idx_badges_user_id ON badges(user_id);        -- For looking up a user's badges
CREATE INDEX idx_badges_date ON badges(date);              -- For chronological badge analysis
CREATE INDEX idx_badges_name ON badges(name);              -- For analyzing specific badge distributions
CREATE INDEX idx_badges_class ON badges(class);            -- For filtering by badge level

-- Posts table - core content storage for questions, answers, wiki posts
-- Central repository for all user-generated content
CREATE TABLE posts (
    id INTEGER PRIMARY KEY,                 -- Unique identifier for each post
    post_type_id SMALLINT NOT NULL,         -- Type of post (1=Question, 2=Answer, etc.)
    accepted_answer_id INTEGER,             -- For questions, the ID of the accepted answer
    creation_date TIMESTAMP NOT NULL,       -- When the post was created
    deletion_date TIMESTAMP,                -- When/if the post was deleted
    score INTEGER DEFAULT 0,                -- Net vote score (upvotes - downvotes)
    view_count INTEGER DEFAULT 0,           -- How many times the post was viewed
    body TEXT,                              -- Main content/text of the post
    owner_user_id INTEGER,                  -- User who created the post
    owner_display_name TEXT,                -- For anonymous/deleted users
    last_editor_user_id INTEGER,            -- Last user to edit
    last_editor_display_name TEXT,          -- For anonymous/deleted editors
    last_edit_date TIMESTAMP,               -- When the post was last edited
    last_activity_date TIMESTAMP,           -- When the post had any activity (edits, comments)
    title TEXT,                             -- Title for questions, NULL for answers
    tags TEXT,                              -- Space-delimited list of tags (questions only)
    answer_count INTEGER DEFAULT 0,         -- Number of answers (questions only)
    comment_count INTEGER DEFAULT 0,        -- Number of comments on this post
    favorite_count INTEGER DEFAULT 0,       -- Number of users who favorited this post
    close_date TIMESTAMP,                   -- When the question was closed, if applicable
    community_owned_date TIMESTAMP,         -- When the post became community owned
    content_license TEXT                    -- License applied to the content
);

-- Indexes on posts table for common query patterns and performance
CREATE INDEX idx_posts_post_type_id ON posts (post_type_id);       -- For filtering by post type
CREATE INDEX idx_posts_accepted_answer_id ON posts (accepted_answer_id); -- For finding accepted answers
CREATE INDEX idx_posts_creation_date ON posts (creation_date);     -- For chronological analysis
CREATE INDEX idx_posts_score ON posts (score);                     -- For sorting by popularity
CREATE INDEX idx_posts_view_count ON posts (view_count);           -- For sorting by views/traffic
CREATE INDEX idx_posts_owner_user_id ON posts (owner_user_id);     -- For finding a user's posts
CREATE INDEX idx_posts_last_editor_user_id ON posts (last_editor_user_id); -- For edit analysis
CREATE INDEX idx_posts_last_activity_date ON posts (last_activity_date); -- For recent activity queries
CREATE INDEX idx_posts_tags ON posts USING GIN (to_tsvector('english', tags)); -- For tag-based full-text search
CREATE INDEX idx_posts_title ON posts USING GIN (to_tsvector('english', title)); -- For title-based full-text search

-- Post history table - tracks all revisions and edits
-- Used for maintaining an audit trail of changes to posts
CREATE TABLE post_historys (
    id INTEGER PRIMARY KEY,
    post_history_type_id SMALLINT NOT NULL,  -- Type of history event (edit, close, etc.)
    post_id INTEGER NOT NULL,                -- Associated post
    revision_guid TEXT,                      -- Unique ID for this revision
    creation_date TIMESTAMP NOT NULL,        -- When this history event occurred
    user_id INTEGER,                         -- User who made the change
    user_display_name TEXT,                  -- For anonymous/deleted users
    comment TEXT,                            -- Edit comment/summary
    text TEXT,                               -- Previous content snapshot or diff
    content_license TEXT                     -- License applied to the history content
);

-- Indexes on post history table
CREATE INDEX idx_post_historys_post_id ON post_historys(post_id); -- For finding a post's history
CREATE INDEX idx_post_historys_user_id ON post_historys(user_id); -- For a user's edit history
CREATE INDEX idx_post_historys_creation_date ON post_historys(creation_date); -- Time-based analysis
CREATE INDEX idx_post_historys_post_history_type_id ON post_historys(post_history_type_id); -- For filtering by event type

-- Post links table - tracks relationships between posts
-- Used for duplicate questions, related content, etc.
CREATE TABLE post_links (
    id INTEGER PRIMARY KEY,
    creation_date TIMESTAMP NOT NULL,         -- When the link was created
    post_id INTEGER NOT NULL,                 -- Source post
    related_post_id INTEGER NOT NULL,         -- Target/linked post
    link_type_id SMALLINT NOT NULL            -- Type of relationship (1=linked, 3=duplicate)
);

-- Indexes on post links table
CREATE INDEX post_links_post_id_idx ON post_links (post_id); -- For finding links from a post
CREATE INDEX post_links_related_post_id_idx ON post_links (related_post_id); -- For finding links to a post
CREATE INDEX post_links_link_type_id_idx ON post_links (link_type_id); -- For filtering by link type
CREATE INDEX post_links_composite_idx ON post_links (post_id, related_post_id); -- For specific link lookups

-- Comments table - stores comments on posts
-- Used for short discussions, clarifications, and feedback
CREATE TABLE comments (
    id INTEGER PRIMARY KEY,
    post_id INTEGER NOT NULL,                 -- Post being commented on
    score INTEGER DEFAULT 0,                  -- Net vote score on the comment
    text TEXT NOT NULL,                       -- Content of the comment
    creation_date TIMESTAMP NOT NULL,         -- When the comment was created
    user_display_name VARCHAR(255),           -- For anonymous/deleted users
    user_id INTEGER,                          -- User who created the comment
    content_license TEXT                      -- License applied to the comment
);

-- Indexes on comments table
CREATE INDEX idx_comments_post_id ON comments(post_id); -- For finding comments on a post
CREATE INDEX idx_comments_user_id ON comments(user_id); -- For finding a user's comments
CREATE INDEX idx_comments_creation_date ON comments(creation_date); -- For chronological analysis
CREATE INDEX idx_comments_score ON comments(score); -- For sorting by popular comments

-- Tags table - stores topic tags used to categorize questions
-- Central repository for all topic tags with usage statistics
CREATE TABLE tags (
    id INTEGER PRIMARY KEY,
    tag_name TEXT NOT NULL,                   -- The tag text (e.g., "javascript")
    count INTEGER NOT NULL DEFAULT 0,         -- Number of posts using this tag
    excerpt_post_id INTEGER,                  -- Tag wiki excerpt post
    wiki_post_id INTEGER,                     -- Tag wiki full post
    is_moderator_only SMALLINT NOT NULL DEFAULT 0, -- Whether only moderators can use this tag
    is_required SMALLINT NOT NULL DEFAULT 0   -- Whether this tag is required
);

-- Indexes on tags table
CREATE UNIQUE INDEX idx_tags_name ON tags(tag_name); -- For tag name lookups
CREATE INDEX idx_tags_count ON tags(count);          -- For sorting by popular tags

-- Votes table - tracks all voting activity
-- Records upvotes, downvotes, favorites, bounties, etc.
CREATE TABLE votes (
    id INTEGER PRIMARY KEY,
    post_id INTEGER NOT NULL,                 -- Post that was voted on
    vote_type_id SMALLINT NOT NULL,           -- Type of vote (2=upvote, 3=downvote, etc.)
    user_id INTEGER,                          -- User who voted
    creation_date TIMESTAMP NOT NULL,         -- When the vote was cast
    bounty_amount INTEGER,                    -- Bounty amount if applicable
    CONSTRAINT chk_bounty_amount CHECK (vote_type_id != 8 OR bounty_amount IS NOT NULL) -- Ensure bounty votes have amount
);

-- Indexes on votes table
CREATE INDEX votes_post_id_idx ON votes (post_id);        -- For finding votes on a post
CREATE INDEX votes_vote_type_id_idx ON votes (vote_type_id); -- For filtering by vote type
CREATE INDEX votes_user_id_idx ON votes (user_id);        -- For finding a user's voting history
CREATE INDEX votes_creation_date_idx ON votes (creation_date); -- For time-based analysis

-- Materialized view for common query patterns in Superset
-- Pre-aggregates post statistics for faster dashboard rendering
CREATE MATERIALIZED VIEW post_stats AS
SELECT 
    p.id, 
    p.title, 
    p.creation_date, 
    p.score,
    p.view_count,
    p.answer_count,
    p.comment_count,
    p.post_type_id,
    u.display_name as author,                -- Post author's display name
    COUNT(DISTINCT v.id) as vote_count,      -- Total number of votes (all types)
    COUNT(DISTINCT c.id) as actual_comment_count, -- Actual comment count from comments table
    p.tags
FROM posts p
LEFT JOIN users u ON p.owner_user_id = u.id
LEFT JOIN votes v ON p.id = v.post_id
LEFT JOIN comments c ON p.id = c.post_id
GROUP BY p.id, p.title, p.creation_date, p.score, p.view_count, 
         p.answer_count, p.comment_count, p.post_type_id, u.display_name, p.tags;

-- Indexes on the materialized view for Superset performance
CREATE UNIQUE INDEX idx_post_stats_id ON post_stats(id);              -- Primary lookup key
CREATE INDEX idx_post_stats_creation_date ON post_stats(creation_date); -- For time-series charts
CREATE INDEX idx_post_stats_score ON post_stats(score);                -- For popularity filtering
CREATE INDEX idx_post_stats_post_type_id ON post_stats(post_type_id);  -- For post type filtering
