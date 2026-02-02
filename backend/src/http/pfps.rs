use crate::http::ApiContext;
use crate::http::extractor::AuthUser;
use crate::http::{Error, Result};
use anyhow::Context;
use axum::extract::{Multipart, Path, State};
use axum::http::{HeaderMap, HeaderValue, header};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Serialize;
use std::path::Path as StdPath;
use uuid::Uuid;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/user/pfp", post(upload_pfp))
        .route("/api/pfps/{pfp_id}", get(get_pfp))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Pfp {
    pfp_id: Uuid,
}

#[derive(Serialize)]
struct PfpBody {
    pfp: Pfp,
}

async fn upload_pfp(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    mut multipart: Multipart,
) -> Result<Json<PfpBody>> {
    let mut raw_file_name = None;
    let mut content_type = None;
    let mut file_bytes = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .context("failed to read multipart field")?
    {
        if file_bytes.is_some() {
            continue;
        }

        let is_file_field = matches!(field.name(), Some("file")) || raw_file_name.is_none();
        if !is_file_field {
            continue;
        }

        raw_file_name = Some(
            field
                .file_name()
                .map(|name| name.to_string())
                .unwrap_or_else(|| "pfp".to_string()),
        );
        content_type = field.content_type().map(|ct| ct.to_string());
        file_bytes = Some(
            field
                .bytes()
                .await
                .context("failed to read profile picture file")?
                .to_vec(),
        );
    }

    let raw_file_name =
        raw_file_name.ok_or_else(|| Error::unprocessable_entity([("file", "missing")]))?;
    let file_bytes =
        file_bytes.ok_or_else(|| Error::unprocessable_entity([("file", "missing")]))?;

    let file_name = sanitize_filename(&raw_file_name);
    let pfp_id = Uuid::new_v4();
    let stored_name = match StdPath::new(&file_name)
        .extension()
        .and_then(|ext| ext.to_str())
    {
        Some(ext) => format!("{}.{}", pfp_id, ext),
        None => pfp_id.to_string(),
    };

    let upload_dir = ctx.config.storage.pfp_upload_dir.clone();
    let file_path = format!("{}/{}", upload_dir, stored_name);
    let size_bytes = file_bytes.len() as i64;

    sqlx::query!(
        r#"
            insert into profile_picture (pfp_id, user_id, file_name, file_path, content_type, size_bytes)
            values ($1, $2, $3, $4, $5, $6)
        "#,
        pfp_id,
        auth_user.user_id,
        file_name,
        file_path,
        content_type,
        size_bytes,
    )
    .execute(&ctx.db)
    .await?;

    let write_path = file_path.clone();
    let write_bytes = file_bytes;
    let write_dir = upload_dir.clone();
    let write_result = tokio::task::spawn_blocking(move || -> std::io::Result<()> {
        std::fs::create_dir_all(&write_dir)?;
        std::fs::write(&write_path, write_bytes)?;
        Ok(())
    })
    .await
    .context("failed to join profile picture write task")?;

    if let Err(err) = write_result {
        sqlx::query!(r#"delete from profile_picture where pfp_id = $1"#, pfp_id)
            .execute(&ctx.db)
            .await?;

        return Err(anyhow::anyhow!(err)
            .context("failed to write profile picture file")
            .into());
    }

    Ok(Json(PfpBody {
        pfp: Pfp { pfp_id },
    }))
}

async fn get_pfp(
    State(ctx): State<ApiContext>,
    Path(pfp_id): Path<Uuid>,
) -> Result<(HeaderMap, Vec<u8>)> {
    let record = sqlx::query!(
        r#"select file_path, file_name, content_type from profile_picture where pfp_id = $1"#,
        pfp_id
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    let file_path = record.file_path;
    let file_name = sanitize_filename(&record.file_name);

    let read_result = tokio::task::spawn_blocking(move || std::fs::read(&file_path)).await;
    let file_bytes = match read_result {
        Ok(Ok(bytes)) => bytes,
        Ok(Err(err)) if err.kind() == std::io::ErrorKind::NotFound => return Err(Error::NotFound),
        Ok(Err(err)) => {
            return Err(anyhow::anyhow!(err)
                .context("failed to read profile picture file")
                .into());
        }
        Err(err) => {
            return Err(anyhow::Error::from(err)
                .context("failed to join profile picture read task")
                .into());
        }
    };

    let mut headers = HeaderMap::new();
    let content_type = record
        .content_type
        .as_deref()
        .unwrap_or("application/octet-stream");
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(content_type).context("failed to build content-type header")?,
    );
    let disposition = format!("inline; filename=\"{}\"", file_name.replace('"', "_"));
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&disposition)
            .context("failed to build content-disposition header")?,
    );

    Ok((headers, file_bytes))
}

fn sanitize_filename(name: &str) -> String {
    StdPath::new(name)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("pfp")
        .to_string()
}
