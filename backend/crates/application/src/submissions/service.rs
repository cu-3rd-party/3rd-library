use std::sync::Arc;

use library_domain::common::{PageRequest, now};
use library_domain::materials::{FileId, FileName, MaterialFile, MaterialInfo};
use library_domain::submissions::{Submission, SubmissionChanges, SubmissionId, SubmissionRepository};
use library_domain::users::{UserId, UserRepository};
use uuid::Uuid;

use super::{CreateSubmission, NewFile, SubmissionDto, UpdateSubmission, parse_status_filter};
use crate::access::is_moderator;
use crate::common::PageDto;
use crate::locks::lock;
use crate::ports::FileStorage;
use crate::{AppError, AppResult};

const ENTITY: &str = "Submission";

/// Заявки автора: подача, просмотр, правка.
#[derive(Clone)]
pub struct SubmissionService {
    users: Arc<dyn UserRepository>,
    submissions: Arc<dyn SubmissionRepository>,
    storage: Arc<dyn FileStorage>,
}

impl SubmissionService {
    pub fn new(
        users: Arc<dyn UserRepository>,
        submissions: Arc<dyn SubmissionRepository>,
        storage: Arc<dyn FileStorage>,
    ) -> Self {
        Self {
            users,
            submissions,
            storage,
        }
    }

    /// Заявки пользователя; `status` — значение статуса или `all`.
    pub async fn list_own(
        &self,
        user_id: Uuid,
        status: Option<&str>,
        page: PageRequest,
    ) -> AppResult<PageDto<SubmissionDto>> {
        let status = parse_status_filter(status, None)?;
        let submissions = self
            .submissions
            .list_by_author(UserId::from(user_id), status, page)
            .await?;
        Ok(PageDto::new(submissions, page, |submission| {
            SubmissionDto::from(&submission)
        }))
    }

    /// Новая заявка с файлами; сразу уходит на модерацию.
    pub async fn create(&self, user_id: Uuid, request: CreateSubmission) -> AppResult<SubmissionDto> {
        let author = UserId::from(user_id);
        let files = self.store_files(author, request.files).await?;
        let info = MaterialInfo {
            title: request.title,
            description: request.description.unwrap_or_default(),
            courses: request.courses,
            subjects: request.subjects,
            kind: request.kind,
            difficulty: request.difficulty,
        };
        let submission = Submission::submit(author, info, files, now());

        self.submissions.insert(&submission).await?;
        Ok(SubmissionDto::from(&submission))
    }

    /// Заявку видят её автор и модераторы.
    pub async fn get(&self, user_id: Uuid, id: Uuid) -> AppResult<SubmissionDto> {
        let submission = self.find(id).await?;
        let user = UserId::from(user_id);
        if !submission.is_owned_by(user) && !is_moderator(self.users.as_ref(), user).await? {
            return Err(AppError::Forbidden);
        }
        Ok(SubmissionDto::from(&submission))
    }

    /// Правка своей заявки в статусе `draft` или `rejected`.
    pub async fn update(&self, user_id: Uuid, id: Uuid, request: UpdateSubmission) -> AppResult<SubmissionDto> {
        let user = UserId::from(user_id);
        let _guard = lock(id).await;
        let mut submission = self.find(id).await?;
        if !submission.is_owned_by(user) || !submission.is_editable() {
            return Err(AppError::Forbidden);
        }

        let new_files = self.store_files(user, request.files).await?;
        submission.edit(
            SubmissionChanges {
                title: request.title,
                description: request.description,
                courses: request.courses,
                subjects: request.subjects,
                kind: request.kind,
                difficulty: request.difficulty,
                keep_file_ids: request
                    .keep_file_ids
                    .map(|ids| ids.into_iter().map(FileId::from).collect()),
                new_files,
            },
            now(),
        )?;

        if !self.submissions.update(&submission).await? {
            return Err(AppError::not_found(ENTITY, id));
        }
        Ok(SubmissionDto::from(&submission))
    }

    async fn find(&self, id: Uuid) -> AppResult<Submission> {
        self.submissions
            .find_by_id(SubmissionId::from(id))
            .await?
            .ok_or_else(|| AppError::not_found(ENTITY, id))
    }

    /// Проверяет все файлы и только потом загружает их в хранилище.
    async fn store_files(&self, owner: UserId, files: Vec<NewFile>) -> AppResult<Vec<MaterialFile>> {
        let mut checked = Vec::with_capacity(files.len());
        for file in files {
            let name = FileName::parse(file.file_name)?;
            let content_type = file
                .content_type
                .ok_or_else(|| AppError::validation("files", "content type is required"))?;
            checked.push((name, content_type, file.content));
        }

        let mut stored = Vec::with_capacity(checked.len());
        for (name, content_type, content) in checked {
            let size = i64::try_from(content.len()).unwrap_or(i64::MAX);
            let file = MaterialFile::new(owner, name, size, Some(content_type.clone()));
            self.storage.put(&file.storage_key, content, &content_type).await?;
            tracing::debug!(key = %file.storage_key, size, "uploaded file");
            stored.push(file);
        }
        Ok(stored)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;
    use bytes::Bytes;
    use library_domain::submissions::SubmissionStatus;

    use chrono::Utc;

    use super::*;
    use crate::ports::ExternalResult;
    use crate::testing::{MockSubmissions, MockUsers};

    #[derive(Default)]
    struct MemoryStorage(Mutex<Vec<String>>);

    #[async_trait]
    impl FileStorage for MemoryStorage {
        async fn put(&self, key: &str, _content: Bytes, _content_type: &str) -> ExternalResult<()> {
            self.0.lock().unwrap().push(key.to_owned());
            Ok(())
        }
    }

    fn service(users: MockUsers, submissions: MockSubmissions, storage: Arc<MemoryStorage>) -> SubmissionService {
        SubmissionService::new(Arc::new(users), Arc::new(submissions), storage)
    }

    fn pdf(name: &str) -> NewFile {
        NewFile {
            file_name: name.into(),
            content_type: Some("application/pdf".into()),
            content: Bytes::from_static(b"%PDF"),
        }
    }

    fn request(files: Vec<NewFile>) -> CreateSubmission {
        CreateSubmission {
            title: "Lecture".into(),
            description: None,
            courses: vec!["1".into()],
            subjects: vec!["math".into()],
            kind: "notes".into(),
            difficulty: "none".into(),
            files,
        }
    }

    fn existing(author: UserId) -> Submission {
        let info = MaterialInfo {
            title: "Lecture".into(),
            description: String::new(),
            courses: vec!["1".into()],
            subjects: vec!["math".into()],
            kind: "notes".into(),
            difficulty: "none".into(),
        };
        Submission::submit(author, info, vec![], Utc::now())
    }

    #[tokio::test]
    async fn create_uploads_files_and_inserts() {
        let mut submissions = MockSubmissions::new();
        submissions
            .expect_insert()
            .withf(|submission| submission.files().len() == 2 && submission.info().description.is_empty())
            .times(1)
            .returning(|_| Ok(()));
        let storage = Arc::new(MemoryStorage::default());

        let dto = service(MockUsers::new(), submissions, storage.clone())
            .create(Uuid::now_v7(), request(vec![pdf("a.pdf"), pdf("a.pdf")]))
            .await
            .unwrap();

        assert_eq!(dto.status, SubmissionStatus::PendingReview);
        let keys = storage.0.lock().unwrap();
        assert_eq!(keys.len(), 2);
        assert_ne!(keys[0], keys[1]);
    }

    #[tokio::test]
    async fn create_with_bad_file_uploads_nothing() {
        let mut submissions = MockSubmissions::new();
        submissions.expect_insert().never();
        let storage = Arc::new(MemoryStorage::default());

        let error = service(MockUsers::new(), submissions, storage.clone())
            .create(Uuid::now_v7(), request(vec![pdf("ok.pdf"), pdf("bad.exe")]))
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::Validation(errors) if errors.contains_key("files")));
        assert!(storage.0.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_foreign_submission_requires_moderator() {
        let submission = existing(UserId::new());
        let mut submissions = MockSubmissions::new();
        submissions
            .expect_find_by_id()
            .returning(move |_| Ok(Some(submission.clone())));
        let mut users = MockUsers::new();
        users.expect_find_by_id().returning(|_| Ok(None));

        let error = service(users, submissions, Default::default())
            .get(Uuid::now_v7(), Uuid::now_v7())
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::Forbidden));
    }

    #[tokio::test]
    async fn update_pending_review_is_forbidden() {
        let author = UserId::new();
        let submission = existing(author);
        let mut submissions = MockSubmissions::new();
        submissions
            .expect_find_by_id()
            .returning(move |_| Ok(Some(submission.clone())));
        submissions.expect_update().never();

        let error = service(MockUsers::new(), submissions, Default::default())
            .update(author.into_inner(), Uuid::now_v7(), UpdateSubmission::default())
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::Forbidden));
    }

    #[tokio::test]
    async fn list_own_rejects_unknown_status() {
        let error = service(MockUsers::new(), MockSubmissions::new(), Default::default())
            .list_own(Uuid::now_v7(), Some("published"), PageRequest::default())
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::Validation(_)));
    }
}
