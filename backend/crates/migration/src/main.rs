use std::collections::HashMap;
use std::env;

use anyhow::Context;
use library_migration::MIGRATOR;
use sqlx::{Connection, PgConnection};

/// `cargo run -p library-migration -- [up|status]`; БД — `DATABASE_URL` или `POSTGRES_*`, как у web.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let mut conn = PgConnection::connect(&database_url())
        .await
        .context("failed to connect to the database")?;

    match env::args().nth(1).as_deref() {
        Some("up") => {
            MIGRATOR.run(&mut conn).await?;
            println!("migrations applied");
        }
        Some("status") | None => status(&mut conn).await?,
        Some(other) => anyhow::bail!("unknown command '{other}', expected 'up' or 'status'"),
    }
    Ok(())
}

async fn status(conn: &mut PgConnection) -> anyhow::Result<()> {
    let applied: HashMap<i64, bool> = sqlx::query_as("select version, success from _sqlx_migrations")
        .fetch_all(&mut *conn)
        .await
        .unwrap_or_default()
        .into_iter()
        .collect();

    for migration in MIGRATOR.iter() {
        let state = match applied.get(&migration.version) {
            Some(true) => "applied",
            Some(false) => "failed",
            None => "pending",
        };
        println!("{:>4} {:<8} {}", migration.version, state, migration.description);
    }
    for version in applied
        .keys()
        .filter(|version| MIGRATOR.iter().all(|m| m.version != **version))
    {
        println!("{version:>4} applied  (no such file — applied by another version of the backend)");
    }
    Ok(())
}

fn database_url() -> String {
    if let Ok(url) = env::var("DATABASE_URL") {
        return url;
    }
    let var = |name: &str, default: &str| env::var(name).unwrap_or_else(|_| default.to_owned());
    format!(
        "postgres://{}:{}@{}:{}/{}",
        var("POSTGRES_USER", "admin"),
        var("POSTGRES_PASSWORD", "admin"),
        var("POSTGRES_HOST", "localhost"),
        var("POSTGRES_PORT", "5432"),
        var("POSTGRES_DB", "db"),
    )
}
