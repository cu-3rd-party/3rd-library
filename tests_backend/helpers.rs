use once_cell::sync::OnceCell;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Row;
use std::time::Duration;

static TEST_POOL: OnceCell<PgPool> = OnceCell::new();

pub async fn test_db_pool() -> &'static PgPool {
    TEST_POOL.get_or_init(|| {
        let db_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
        
        PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(30))
            .lazy(true)
    });
    
    TEST_POOL.get().unwrap()
}

pub async fn create_test_tables(pool: &PgPool) -> sqlx::Result<()> {
    sqlx::query(r#"
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
        )
    "#).execute(pool).await?;

    sqlx::query(r#"
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
        )
    "#).execute(pool).await?;

    sqlx::query(r#"
        create table if not exists material_file (
            file_id uuid default uuid_generate_v1mc() primary key,
            user_id uuid references web_user(user_id) not null,
            name text not null,
            size_bytes bigint not null,
            extension text not null,
            mime_type text,
            storage_key text not null,
            created_at timestamptz default now()
        )
    "#).execute(pool).await?;

    sqlx::query(r#"
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
        )
    "#).execute(pool).await?;

    Ok(())
}

pub async fn cleanup_test_tables(pool: &PgPool) -> sqlx::Result<()> {
    sqlx::query("delete from submission").execute(pool).await?;
    sqlx::query("delete from material_file_rel").execute(pool).await?;
    sqlx::query("delete from material_file").execute(pool).await?;
    sqlx::query("delete from material").execute(pool).await?;
    sqlx::query("delete from web_user").execute(pool).await?;
    Ok(())
}