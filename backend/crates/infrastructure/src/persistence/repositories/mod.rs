mod material_repository;
mod submission_repository;
mod user_repository;

pub use material_repository::SqlxMaterialRepository;
pub use submission_repository::SqlxSubmissionRepository;
pub use user_repository::SqlxUserRepository;

use library_domain::RepositoryError;

fn db_error(error: sqlx::Error) -> RepositoryError {
    RepositoryError::new(error)
}

/// Шаблон `ilike` для поиска подстроки.
fn contains_pattern(search: Option<String>) -> Option<String> {
    search.map(|search| format!("%{search}%"))
}
