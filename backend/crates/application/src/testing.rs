//! Моки репозиториев для юнит-тестов сервисов.

use async_trait::async_trait;
use library_domain::common::{Page, PageRequest};
use library_domain::error::RepositoryResult;
use library_domain::materials::{Material, MaterialFile, MaterialId, MaterialRepository, MaterialSummary};
use library_domain::submissions::{StatusCounters, Submission, SubmissionId, SubmissionRepository, SubmissionStatus};
use library_domain::users::{AuthorSummary, User, UserId, UserRepository};
use mockall::mock;

mock! {
    pub Users {}

    #[async_trait]
    impl UserRepository for Users {
        async fn find_by_id(&self, id: UserId) -> RepositoryResult<Option<User>>;
        async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>>;
        async fn insert(&self, user: &User) -> RepositoryResult<bool>;
        async fn update(&self, user: &User) -> RepositoryResult<bool>;
        async fn list_authors(&self, search: Option<String>, page: PageRequest) -> RepositoryResult<Page<AuthorSummary>>;
    }
}

mock! {
    pub Materials {}

    #[async_trait]
    impl MaterialRepository for Materials {
        async fn list_published(&self, search: Option<String>, page: PageRequest) -> RepositoryResult<Page<MaterialSummary>>;
        async fn list_published_by_author(&self, author: UserId, page: PageRequest) -> RepositoryResult<Page<MaterialSummary>>;
        async fn find_by_id(&self, id: MaterialId) -> RepositoryResult<Option<MaterialSummary>>;
        async fn files(&self, id: MaterialId) -> RepositoryResult<Vec<MaterialFile>>;
        async fn insert(&self, material: &Material) -> RepositoryResult<()>;
    }
}

mock! {
    pub Submissions {}

    #[async_trait]
    impl SubmissionRepository for Submissions {
        async fn find_by_id(&self, id: SubmissionId) -> RepositoryResult<Option<Submission>>;
        async fn list_by_author(&self, author: UserId, status: Option<SubmissionStatus>, page: PageRequest) -> RepositoryResult<Page<Submission>>;
        async fn list_for_moderation(&self, status: Option<SubmissionStatus>, page: PageRequest) -> RepositoryResult<Page<Submission>>;
        async fn count_by_status(&self) -> RepositoryResult<StatusCounters>;
        async fn insert(&self, submission: &Submission) -> RepositoryResult<()>;
        async fn update(&self, submission: &Submission) -> RepositoryResult<bool>;
    }
}
