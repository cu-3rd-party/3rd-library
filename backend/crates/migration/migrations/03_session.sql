-- Сессии пользователей (раньше жили в Redis). Id сессии — он же refresh-токен.
create table if not exists session (
    session_id uuid primary key,
    user_id uuid not null references web_user(user_id) on delete cascade,
    verified bool not null,
    expires_at timestamptz not null,
    created_at timestamptz not null default now()
);
create index if not exists idx_session_expires_at on session(expires_at);
