use axum::extract::State;
use library_application::common::{PageDto, PageRequest};
use library_application::materials::{MaterialDetailsDto, MaterialDto, MaterialFileDto};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;

use crate::AppState;
use crate::auth::MaybeAuthUser;
use crate::error::{ApiResult, ErrorBody};
use crate::extract::{Json, Path, Query};
use crate::routes::timestamp;

pub(crate) const TAG: &str = "materials";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_materials))
        .routes(routes!(get_material))
}

#[derive(Serialize, ToSchema)]
pub struct Material {
    pub id: Uuid,
    pub author_id: Uuid,
    pub author_name: Option<String>,
    pub title: String,
    pub description: String,
    pub courses: Vec<String>,
    pub subjects: Vec<String>,
    #[serde(rename = "type")]
    pub kind: String,
    pub difficulty: String,
    pub pub_date: Option<String>,
}

impl From<MaterialDto> for Material {
    fn from(dto: MaterialDto) -> Self {
        Self {
            id: dto.id,
            author_id: dto.author_id,
            author_name: Some(dto.author_name),
            title: dto.info.title,
            description: dto.info.description,
            courses: dto.info.courses,
            subjects: dto.info.subjects,
            kind: dto.info.kind,
            difficulty: dto.info.difficulty,
            pub_date: dto.published_at.map(timestamp),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct MaterialFile {
    pub id: Uuid,
    pub name: String,
    pub size_bytes: i64,
    pub extension: String,
    pub mime_type: Option<String>,
    pub url: Option<String>,
}

impl From<MaterialFileDto> for MaterialFile {
    fn from(dto: MaterialFileDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            size_bytes: dto.size_bytes,
            extension: dto.extension,
            mime_type: dto.mime_type,
            url: None,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct MaterialDetails {
    #[serde(flatten)]
    pub material: Material,
    pub files: Vec<MaterialFile>,
    pub published_at: Option<String>,
    pub submitted_at: Option<String>,
}

impl From<MaterialDetailsDto> for MaterialDetails {
    fn from(dto: MaterialDetailsDto) -> Self {
        let published_at = dto.material.published_at.map(timestamp);
        let submitted_at = Some(timestamp(dto.material.created));
        Self {
            material: dto.material.into(),
            files: dto.files.into_iter().map(Into::into).collect(),
            published_at,
            submitted_at,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedMaterialsResponse {
    pub items: Vec<Material>,
    pub page: i64,
    pub limit: i64,
    pub total: i64,
}

impl From<PageDto<MaterialDto>> for PaginatedMaterialsResponse {
    fn from(page: PageDto<MaterialDto>) -> Self {
        Self {
            items: page.items.into_iter().map(Into::into).collect(),
            page: page.page,
            limit: page.limit,
            total: page.total,
        }
    }
}

/// Фильтры кроме `search` пока не поддерживаются.
#[derive(Deserialize, IntoParams, ToSchema)]
pub struct ListMaterialsQuery {
    pub search: Option<String>,
    pub courses: Option<String>,
    pub subjects: Option<String>,
    pub types: Option<String>,
    pub difficulties: Option<String>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[utoipa::path(get, path = "/api/materials", tag = TAG, params(ListMaterialsQuery),
    responses((status = 200, description = "Published materials list", body = PaginatedMaterialsResponse)),
    security((), ("bearer_auth" = [])))]
async fn list_materials(
    _user: MaybeAuthUser,
    State(state): State<AppState>,
    Query(query): Query<ListMaterialsQuery>,
) -> ApiResult<Json<PaginatedMaterialsResponse>> {
    let page = PageRequest::new(query.page, query.limit);
    Ok(Json(state.materials.list(query.search, page).await?.into()))
}

#[utoipa::path(get, path = "/api/materials/{materialId}", tag = TAG,
    params(("materialId" = Uuid, Path, description = "Material id")),
    responses(
        (status = 200, description = "Material details", body = MaterialDetails),
        (status = 404, description = "Material not found", body = ErrorBody),
    ))]
async fn get_material(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<MaterialDetails>> {
    Ok(Json(state.materials.get(id).await?.into()))
}
