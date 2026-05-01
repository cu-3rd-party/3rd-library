use crate::http::{ApiContext, Result};
use axum::Json;
use axum::extract::{Path, State};

use crate::http::error::Error;
use sqlx::{Executor, Row};
use time::format_description::well_known::Rfc3339;

use super::models::*;

fn to_rfc3339(dt: time::PrimitiveDateTime) -> String {
    dt.format(&Rfc3339).unwrap().to_string()
}

pub async fn get_material(
    Path(material_id): Path<uuid::Uuid>,
    State(ctx): State<ApiContext>,
) -> Result<Json<MaterialDetails>> {
    let row = sqlx::query(
        r#"
            select
                m.material_id,
                m.user_id,
                u.name as author_name,
                m.title,
                m.description,
                m.courses,
                m.subjects,
                m.type,
                m.difficulty,
                m.published_at,
                m.created_at as submitted_at
            from material m
            inner join web_user u on m.user_id = u.user_id
            where m.material_id = $1
        "#,
    )
    .bind(material_id)
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    let files_rows = sqlx::query(
        r#"
            select mf.file_id, mf.name, mf.size_bytes, mf.extension, mf.mime_type
            from material_file mf
            inner join material_file_rel mfr on mf.file_id = mfr.file_id
            where mfr.material_id = $1
        "#,
    )
    .bind(material_id)
    .fetch_all(&ctx.db)
    .await?;

    let files: Vec<MaterialFile> = files_rows
        .into_iter()
        .map(|f| MaterialFile {
            id: f.get::<uuid::Uuid, _>("file_id"),
            name: f.get::<String, _>("name"),
            size_bytes: f.get::<i64, _>("size_bytes"),
            extension: f.get::<String, _>("extension"),
            mime_type: f.get::<Option<String>, _>("mime_type"),
            url: None,
        })
        .collect();

    let published_at: Option<String> = row
        .get::<Option<time::PrimitiveDateTime>, _>("published_at")
        .map(to_rfc3339);
    let submitted_at: Option<String> = row
        .get::<Option<time::PrimitiveDateTime>, _>("submitted_at")
        .map(to_rfc3339);

    Ok(Json(MaterialDetails {
        material: Material {
            id: row.get::<uuid::Uuid, _>("material_id"),
            author_id: row.get::<uuid::Uuid, _>("user_id"),
            author_name: row.get::<Option<String>, _>("author_name"),
            title: row.get::<String, _>("title"),
            description: row
                .get::<Option<String>, _>("description")
                .unwrap_or_default(),
            courses: row
                .get::<Option<Vec<String>>, _>("courses")
                .unwrap_or_default(),
            subjects: row
                .get::<Option<Vec<String>>, _>("subjects")
                .unwrap_or_default(),
            r#type: row.get::<String, _>("type"),
            difficulty: row
                .get::<Option<String>, _>("difficulty")
                .unwrap_or_else(|| "none".to_string()),
            pub_date: published_at.clone(),
        },
        files,
        published_at,
        submitted_at,
    }))
}
