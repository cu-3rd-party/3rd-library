-- Materials system: tables for the 3rd Library API

-- User table with roles and verification
create table if not exists web_user (
    user_id uuid default uuid_generate_v1mc() primary key,
    email text not null unique,
    password_hash text not null,
    name text not null,
    bio text default '',
    roles text[] default array ['user'],
    is_email_verified bool default false,
    verification_code text,
    verification_code_expires_at timestamptz,
    created_at timestamptz default now(),
    updated_at timestamptz default now()
);
select trigger_updated_at('web_user');

-- Material files (stored as attachments)
create table if not exists material_file (
    file_id uuid default uuid_generate_v1mc() primary key,
    user_id uuid references web_user(user_id) not null,
    name text not null,
    size_bytes bigint not null,
    extension text not null,
    mime_type text,
    storage_key text not null,
    created_at timestamptz default now()
);
select trigger_updated_at('material_file');

-- Materials (published approved materials)
create table if not exists material (
    material_id uuid default uuid_generate_v1mc() primary key,
    user_id uuid references web_user(user_id) not null,
    title text not null,
    description text default '',
    courses text[] not null,
    subjects text[] not null,
    type text not null,
    difficulty text not null default 'none',
    published_at timestamptz,
    created_at timestamptz default now(),
    updated_at timestamptz default now()
);
select trigger_updated_at('material');

-- Material files relation
create table if not exists material_file_rel (
    material_id uuid references material(material_id) not null,
    file_id uuid references material_file(file_id) not null,
    primary key (material_id, file_id)
);

-- Submissions (materials pending review or drafts)
create table if not exists submission (
    submission_id uuid default uuid_generate_v1mc() primary key,
    user_id uuid references web_user(user_id) not null,
    material_id uuid references material(material_id),
    title text not null,
    description text default '',
    courses text[] not null,
    subjects text[] not null,
    type text not null,
    difficulty text not null default 'none',
    status text not null default 'draft',
    moderator_comment text default '',
    created_at timestamptz default now(),
    updated_at timestamptz default now(),
    submitted_at timestamptz,
    reviewed_at timestamptz
);
select trigger_updated_at('submission');

-- Submission files relation
create table if not exists submission_file_rel (
    submission_id uuid references submission(submission_id) not null,
    file_id uuid references material_file(file_id) not null,
    primary key (submission_id, file_id)
);

-- Indexes
create index if not exists idx_material_user on material(user_id);
create index if not exists idx_material_published on material(published_at) where published_at is not null;
create index if not exists idx_submission_user on submission(user_id);
create index if not exists idx_submission_status on submission(status);
create index if not exists idx_web_user_email on web_user(email);