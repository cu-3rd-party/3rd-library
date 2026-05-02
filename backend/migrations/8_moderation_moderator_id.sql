-- Add moderator tracking to submission moderation

alter table submission
add column if not exists moderator_id uuid references web_user(user_id);

create index if not exists idx_submission_moderator on submission(moderator_id);