use crate::http::{ApiContext, Result};
use axum::Json;
use axum::extract::{Query, State};

use crate::http::error::Error;
use crate::http::extractor::{AuthUser, MaybeAuthUser};
use sqlx::Row;
use time::format_description::well_known::Rfc3339;

use super::models::*;

fn to_rfc3339(dt: time::PrimitiveDateTime) -> String {
    dt.format(&Rfc3339).unwrap().to_string()
}

pub async fn list_materials(
    Query(query): Query<ListMaterialsQuery>,
    _maybe_auth_user: MaybeAuthUser,
    State(ctx): State<ApiContext>,
) -> Result<Json<PaginatedMaterialsResponse>> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    let search_pattern = query.search.map(|s| format!("%{}%", s));

    let total: i64 = sqlx::query_scalar(
        r#"
            select count(*)::int8 from material
            where published_at is not null
            and ($1::text is null or title ilike $1)
        "#,
    )
    .bind(&search_pattern)
    .fetch_one(&ctx.db)
    .await?;

    let limit_plus_one = limit + 1;
    let rows = sqlx::query(
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
                m.published_at
            from material m
            inner join web_user u on m.user_id = u.user_id
            where m.published_at is not null
            and ($1::text is null or m.title ilike $1)
            order by m.published_at desc
            limit $2 offset $3
        "#,
    )
    .bind(&search_pattern)
    .bind(limit_plus_one)
    .bind(offset)
    .fetch_all(&ctx.db)
    .await?;

    let items: Vec<Material> = rows
        .into_iter()
        .take(limit as usize)
        .map(|m| {
            let pub_date: Option<String> = m
                .get::<Option<time::PrimitiveDateTime>, _>("published_at")
                .map(to_rfc3339);
            Material {
                id: m.get::<uuid::Uuid, _>("material_id"),
                author_id: m.get::<uuid::Uuid, _>("user_id"),
                author_name: m.get::<Option<String>, _>("author_name"),
                title: m.get::<String, _>("title"),
                description: m
                    .get::<Option<String>, _>("description")
                    .unwrap_or_default(),
                courses: m
                    .get::<Option<Vec<String>>, _>("courses")
                    .unwrap_or_default(),
                subjects: m
                    .get::<Option<Vec<String>>, _>("subjects")
                    .unwrap_or_default(),
                r#type: m.get::<String, _>("type"),
                difficulty: m
                    .get::<Option<String>, _>("difficulty")
                    .unwrap_or_else(|| "none".to_string()),
                pub_date,
            }
        })
        .collect();

    Ok(Json(PaginatedMaterialsResponse {
        items,
        page,
        limit,
        total,
    }))
}

pub async fn list_submissions(
    auth_user: AuthUser,
    Query(query): Query<ListSubmissionsQuery>,
    State(ctx): State<ApiContext>,
) -> Result<Json<PaginatedSubmissionsResponse>> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    let total: i64 = if query.status.as_deref() == Some("all") || query.status.is_none() {
        sqlx::query_scalar("select count(*)::int8 from submission s where s.user_id = $1")
            .bind(auth_user.user_id)
            .fetch_one(&ctx.db)
            .await?
    } else {
        sqlx::query_scalar(
            "select count(*)::int8 from submission s where s.user_id = $1 and s.status = $2",
        )
        .bind(auth_user.user_id)
        .bind(query.status.as_ref().unwrap())
        .fetch_one(&ctx.db)
        .await?
    };

    let limit_plus_one = limit + 1;
    let rows = sqlx::query(
        r#"
            select
                s.submission_id,
                s.title,
                s.description,
                s.courses,
                s.subjects,
                s.type,
                s.difficulty,
                s.status,
                s.moderator_comment,
                s.created_at,
                s.updated_at,
                s.submitted_at,
                s.reviewed_at,
                s.published_at
            from submission s
            where s.user_id = $1
            order by s.created_at desc
            limit $2 offset $3
        "#,
    )
    .bind(auth_user.user_id)
    .bind(limit_plus_one)
    .bind(offset)
    .fetch_all(&ctx.db)
    .await?;

    let items: Vec<Submission> = rows
        .into_iter()
        .take(limit as usize)
        .map(|s| {
            let pub_date: Option<String> = s
                .get::<Option<time::PrimitiveDateTime>, _>("published_at")
                .map(to_rfc3339);
            Submission {
                id: s.get::<uuid::Uuid, _>("submission_id"),
                material: Material {
                    id: uuid::Uuid::nil(),
                    author_id: auth_user.user_id,
                    author_name: None,
                    title: s.get::<String, _>("title"),
                    description: s
                        .get::<Option<String>, _>("description")
                        .unwrap_or_default(),
                    courses: s
                        .get::<Option<Vec<String>>, _>("courses")
                        .unwrap_or_default(),
                    subjects: s
                        .get::<Option<Vec<String>>, _>("subjects")
                        .unwrap_or_default(),
                    r#type: s.get::<String, _>("type"),
                    difficulty: s
                        .get::<Option<String>, _>("difficulty")
                        .unwrap_or_else(|| "none".to_string()),
                    pub_date,
                },
                files: vec![],
                status: s.get::<String, _>("status"),
                moderator_comment: s
                    .get::<Option<String>, _>("moderator_comment")
                    .unwrap_or_default(),
                created_at: to_rfc3339(s.get::<time::PrimitiveDateTime, _>("created_at")),
                updated_at: to_rfc3339(s.get::<time::PrimitiveDateTime, _>("updated_at")),
                submitted_at: s
                    .get::<Option<time::PrimitiveDateTime>, _>("submitted_at")
                    .map(to_rfc3339),
                reviewed_at: s
                    .get::<Option<time::PrimitiveDateTime>, _>("reviewed_at")
                    .map(to_rfc3339),
                published_at: s
                    .get::<Option<time::PrimitiveDateTime>, _>("published_at")
                    .map(to_rfc3339),
            }
        })
        .collect();

    Ok(Json(PaginatedSubmissionsResponse {
        items,
        page,
        limit,
        total,
    }))
}

pub async fn create_submission(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Json(req): Json<CreateSubmissionRequest>,
) -> Result<Json<Submission>> {
    let user_row = sqlx::query("select is_email_verified from web_user where user_id = $1")
        .bind(auth_user.user_id)
        .fetch_one(&ctx.db)
        .await?;

    if !user_row.get::<bool, _>("is_email_verified") {
        return Err(Error::forbidden(
            "email_not_verified",
            "Email is not verified",
        ));
    }

    let submission_id = uuid::Uuid::new_v4();
    let now = time::OffsetDateTime::now_utc();

    sqlx::query(
        r#"insert into submission (submission_id, user_id, title, description, courses, subjects, type, difficulty, status, submitted_at)
           values ($1, $2, $3, $4, $5, $6, $7, $8, 'pending_review', $9)"#
    )
    .bind(submission_id)
    .bind(auth_user.user_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.courses)
    .bind(&req.subjects)
    .bind(&req.r#type)
    .bind(&req.difficulty)
    .bind(now)
    .execute(&ctx.db)
    .await?;

    Ok(Json(Submission {
        id: submission_id,
        material: Material {
            id: uuid::Uuid::nil(),
            author_id: auth_user.user_id,
            author_name: None,
            title: req.title,
            description: req.description.unwrap_or_default(),
            courses: req.courses,
            subjects: req.subjects,
            r#type: req.r#type,
            difficulty: req.difficulty,
            pub_date: None,
        },
        files: vec![],
        status: "pending_review".to_string(),
        moderator_comment: String::new(),
        created_at: now.format(&Rfc3339).unwrap().to_string(),
        updated_at: now.format(&Rfc3339).unwrap().to_string(),
        submitted_at: Some(now.format(&Rfc3339).unwrap().to_string()),
        reviewed_at: None,
        published_at: None,
    }))
}
