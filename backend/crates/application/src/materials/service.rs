use std::sync::Arc;

use library_domain::common::PageRequest;
use library_domain::materials::{MaterialId, MaterialRepository};
use uuid::Uuid;

use super::{MaterialDetailsDto, MaterialDto, MaterialFileDto};
use crate::common::PageDto;
use crate::{AppError, AppResult};

const ENTITY: &str = "Material";

/// Каталог опубликованных материалов.
#[derive(Clone)]
pub struct MaterialService {
    materials: Arc<dyn MaterialRepository>,
}

impl MaterialService {
    pub fn new(materials: Arc<dyn MaterialRepository>) -> Self {
        Self { materials }
    }

    pub async fn list(&self, search: Option<String>, page: PageRequest) -> AppResult<PageDto<MaterialDto>> {
        let materials = self.materials.list_published(search, page).await?;
        Ok(PageDto::new(materials, page, MaterialDto::from))
    }

    pub async fn get(&self, id: Uuid) -> AppResult<MaterialDetailsDto> {
        let material_id = MaterialId::from(id);
        let material = self
            .materials
            .find_by_id(material_id)
            .await?
            .ok_or_else(|| AppError::not_found(ENTITY, id))?;
        let files = self.materials.files(material_id).await?;

        Ok(MaterialDetailsDto {
            material: material.into(),
            files: files.iter().map(MaterialFileDto::from).collect(),
        })
    }
}
