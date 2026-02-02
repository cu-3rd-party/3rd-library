create table if not exists profile_picture (
    pfp_id uuid primary key default uuid_generate_v1mc(),
    user_id uuid not null references "user" (user_id) on delete cascade,
    file_name text not null,
    file_path text not null,
    content_type text,
    size_bytes bigint,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

select trigger_updated_at('profile_picture');

create index if not exists profile_picture_user_id_idx on profile_picture (user_id);

alter table "user"
    add column if not exists pfp_id uuid;

alter table "user"
    add constraint user_pfp_id_fkey
    foreign key (pfp_id) references profile_picture (pfp_id) on delete set null;

create index if not exists user_pfp_id_idx on "user" (pfp_id);
