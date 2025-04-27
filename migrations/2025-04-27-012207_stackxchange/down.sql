-- This file should undo anything in `up.sql`

-- Drop materialized view and its indexes first
DROP INDEX IF EXISTS idx_post_stats_id;
DROP INDEX IF EXISTS idx_post_stats_creation_date;
DROP INDEX IF EXISTS idx_post_stats_score;
DROP INDEX IF EXISTS idx_post_stats_post_type_id;
DROP MATERIALIZED VIEW IF EXISTS post_stats;

-- Drop votes table and its indexes
DROP INDEX IF EXISTS votes_creation_date_idx;
DROP INDEX IF EXISTS votes_user_id_idx;
DROP INDEX IF EXISTS votes_vote_type_id_idx;
DROP INDEX IF EXISTS votes_post_id_idx;
DROP TABLE IF EXISTS votes;

-- Drop tags table and its indexes
DROP INDEX IF EXISTS idx_tags_name;
DROP INDEX IF EXISTS idx_tags_count;
DROP TABLE IF EXISTS tags;

-- Drop comments table and its indexes
DROP INDEX IF EXISTS idx_comments_creation_date;
DROP INDEX IF EXISTS idx_comments_user_id;
DROP INDEX IF EXISTS idx_comments_post_id;
DROP INDEX IF EXISTS idx_comments_score;
DROP TABLE IF EXISTS comments;

-- Drop post_links table and its indexes
DROP INDEX IF EXISTS post_links_post_id_idx;
DROP INDEX IF EXISTS post_links_related_post_id_idx;
DROP INDEX IF EXISTS post_links_link_type_id_idx;
DROP INDEX IF EXISTS post_links_composite_idx;
DROP TABLE IF EXISTS post_links;

-- Drop post_historys table and its indexes
DROP INDEX IF EXISTS idx_post_historys_post_history_type_id;
DROP INDEX IF EXISTS idx_post_historys_creation_date;
DROP INDEX IF EXISTS idx_post_historys_user_id;
DROP INDEX IF EXISTS idx_post_historys_post_id;
DROP TABLE IF EXISTS post_historys;

-- Drop posts table and its indexes
DROP INDEX IF EXISTS idx_posts_last_editor_user_id;
DROP INDEX IF EXISTS idx_posts_owner_user_id;
DROP INDEX IF EXISTS idx_posts_view_count;
DROP INDEX IF EXISTS idx_posts_score;
DROP INDEX IF EXISTS idx_posts_creation_date;
DROP INDEX IF EXISTS idx_posts_accepted_answer_id;
DROP INDEX IF EXISTS idx_posts_post_type_id;
DROP TABLE IF EXISTS posts;

-- Drop badges table and its indexes
DROP INDEX IF EXISTS idx_badges_user_id;
DROP TABLE IF EXISTS badges;

-- Drop users table and its indexes (last since other tables depend on it)
DROP INDEX IF EXISTS idx_users_reputation;
DROP INDEX IF EXISTS idx_users_creation_date;
DROP INDEX IF EXISTS idx_users_last_access_date;
DROP INDEX IF EXISTS idx_users_up_votes;
DROP INDEX IF EXISTS idx_users_down_votes;
DROP TABLE IF EXISTS users;
