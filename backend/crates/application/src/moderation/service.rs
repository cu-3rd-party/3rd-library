use std::sync::Arc;

use chrono::Utc;
use library_domain::common::PageRequest;
use library_domain::submissions::{StatusCounters, SubmissionId, SubmissionRepository, SubmissionStatus};
use library_domain::unit_of_work::UnitOfWork;
use library_domain::users::{UserId, UserRepository};
use uuid::Uuid;

use crate::access::is_moderator;
use crate::common::PageDto;
use crate::submissions::{SubmissionDto, parse_status_filter};
use crate::{AppError, AppResult};

const ENTITY: &str = "Submission";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModerationQueueDto {
    pub submissions: PageDto<SubmissionDto>,
    pub counters: StatusCounters,
}

/// Очередь модерации и решения по заявкам. Всё — только для модераторов.
#[derive(Clone)]
pub struct ModerationService {
    users: Arc<dyn UserRepository>,
    submissions: Arc<dyn SubmissionRepository>,
    unit_of_work: Arc<dyn UnitOfWork>,
}

impl ModerationService {
    pub fn new(
        users: Arc<dyn UserRepository>,
        submissions: Arc<dyn SubmissionRepository>,
        unit_of_work: Arc<dyn UnitOfWork>,
    ) -> Self {
        Self {
            users,
            submissions,
            unit_of_work,
        }
    }

    /// Заявки со статусом `status` (по умолчанию `pending_review`, `all` — все) и счётчики по статусам.
    pub async fn queue(
        &self,
        moderator: Uuid,
        status: Option<&str>,
        page: PageRequest,
    ) -> AppResult<ModerationQueueDto> {
        self.ensure_moderator(moderator).await?;
        let status = parse_status_filter(status, Some(SubmissionStatus::PendingReview))?;

        let submissions = self.submissions.list_for_moderation(status, page).await?;
        let counters = self.submissions.count_by_status().await?;
        Ok(ModerationQueueDto {
            submissions: PageDto::new(submissions, page, |submission| SubmissionDto::from(&submission)),
            counters,
        })
    }

    /// Решение по заявке: `approve` публикует материал, `reject` требует комментарий.
    pub async fn decide(
        &self,
        moderator: Uuid,
        id: Uuid,
        action: &str,
        comment: Option<String>,
    ) -> AppResult<SubmissionDto> {
        self.ensure_moderator(moderator).await?;
        let moderator = UserId::from(moderator);
        let mut submission = self
            .submissions
            .find_by_id(SubmissionId::from(id))
            .await?
            .ok_or_else(|| AppError::not_found(ENTITY, id))?;
        let now = Utc::now();

        match action {
            "approve" => {
                let material = submission.approve(moderator, now)?;
                // материал и заявка (ссылается на материал) сохраняются вместе или никак
                let transaction = self.unit_of_work.begin().await?;
                transaction.materials().insert(&material).await?;
                if !transaction.submissions().update(&submission).await? {
                    return Err(AppError::not_found(ENTITY, id));
                }
                transaction.commit().await?;
            }
            "reject" => {
                let comment =
                    comment.ok_or_else(|| AppError::validation("moderator_comment", "is required for reject"))?;
                submission.reject(moderator, comment, now)?;
                if !self.submissions.update(&submission).await? {
                    return Err(AppError::not_found(ENTITY, id));
                }
            }
            _ => return Err(AppError::validation("action", "must be 'approve' or 'reject'")),
        }
        Ok(SubmissionDto::from(&submission))
    }

    async fn ensure_moderator(&self, user: Uuid) -> AppResult<()> {
        if !is_moderator(self.users.as_ref(), UserId::from(user)).await? {
            return Err(AppError::Forbidden);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;
    use library_domain::common::Page;
    use library_domain::error::RepositoryResult;
    use library_domain::materials::{MaterialInfo, MaterialRepository};
    use library_domain::submissions::Submission;
    use library_domain::unit_of_work::Transaction;
    use library_domain::users::{Email, PasswordHash, User, UserData, VerificationCode};

    use super::*;
    use crate::testing::{MockMaterials, MockSubmissions, MockUsers};

    /// Журнал того, что произошло в транзакциях.
    #[derive(Default)]
    struct Journal(Mutex<Vec<String>>);

    impl Journal {
        fn push(&self, entry: impl Into<String>) {
            self.0.lock().unwrap().push(entry.into());
        }
    }

    struct RecordingUow(Arc<Journal>);

    #[async_trait]
    impl UnitOfWork for RecordingUow {
        async fn begin(&self) -> RepositoryResult<Box<dyn Transaction>> {
            Ok(Box::new(RecordingTransaction(self.0.clone())))
        }
    }

    struct RecordingTransaction(Arc<Journal>);

    #[async_trait]
    impl Transaction for RecordingTransaction {
        fn users(&self) -> Box<dyn UserRepository + '_> {
            Box::new(MockUsers::new())
        }

        fn materials(&self) -> Box<dyn MaterialRepository + '_> {
            let journal = self.0.clone();
            let mut materials = MockMaterials::new();
            materials.expect_insert().returning(move |material| {
                journal.push(format!("material:{}", material.info().title));
                Ok(())
            });
            Box::new(materials)
        }

        fn submissions(&self) -> Box<dyn SubmissionRepository + '_> {
            let journal = self.0.clone();
            let mut submissions = MockSubmissions::new();
            submissions.expect_update().returning(move |submission| {
                journal.push(format!("submission:{}", submission.status()));
                Ok(true)
            });
            Box::new(submissions)
        }

        async fn commit(self: Box<Self>) -> RepositoryResult<()> {
            self.0.push("commit");
            Ok(())
        }

        async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
            self.0.push("rollback");
            Ok(())
        }
    }

    fn user_with_roles(moderator: bool) -> User {
        let now = Utc::now();
        let mut roles = vec!["user".to_owned()];
        if moderator {
            roles.push("moderator".to_owned());
        }
        let registered = User::register(
            Email::parse("m@example.com").unwrap(),
            "Moderator".into(),
            PasswordHash::new("hash".into()),
            VerificationCode::generate(),
            now,
        );
        User::restore(UserData {
            id: registered.id(),
            email: registered.email().clone(),
            password_hash: registered.password_hash().clone(),
            name: registered.name().into(),
            bio: String::new(),
            roles,
            is_email_verified: true,
            verification_code: None,
            code_issued_at: None,
            code_expires_at: None,
            audit: *registered.audit(),
        })
    }

    fn users(moderator: bool) -> MockUsers {
        let user = user_with_roles(moderator);
        let mut users = MockUsers::new();
        users.expect_find_by_id().returning(move |_| Ok(Some(user.clone())));
        users
    }

    fn pending() -> Submission {
        let info = MaterialInfo {
            title: "Lecture".into(),
            description: String::new(),
            courses: vec!["1".into()],
            subjects: vec!["math".into()],
            kind: "notes".into(),
            difficulty: "none".into(),
        };
        Submission::submit(UserId::new(), info, vec![], Utc::now())
    }

    fn submissions_with(submission: Submission) -> MockSubmissions {
        let mut submissions = MockSubmissions::new();
        submissions
            .expect_find_by_id()
            .returning(move |_| Ok(Some(submission.clone())));
        submissions
    }

    fn service(users: MockUsers, submissions: MockSubmissions, journal: Arc<Journal>) -> ModerationService {
        ModerationService::new(Arc::new(users), Arc::new(submissions), Arc::new(RecordingUow(journal)))
    }

    #[tokio::test]
    async fn not_moderator_is_forbidden() {
        let service = service(users(false), MockSubmissions::new(), Default::default());

        let queue = service.queue(Uuid::now_v7(), None, PageRequest::default()).await;
        let decide = service.decide(Uuid::now_v7(), Uuid::now_v7(), "approve", None).await;

        assert!(matches!(queue, Err(AppError::Forbidden)));
        assert!(matches!(decide, Err(AppError::Forbidden)));
    }

    #[tokio::test]
    async fn queue_defaults_to_pending_review() {
        let mut submissions = MockSubmissions::new();
        submissions
            .expect_list_for_moderation()
            .withf(|status, _| *status == Some(SubmissionStatus::PendingReview))
            .returning(|_, _| {
                Ok(Page {
                    items: vec![],
                    total: 0,
                })
            });
        submissions
            .expect_count_by_status()
            .returning(|| Ok(StatusCounters::default()));

        let queue = service(users(true), submissions, Default::default())
            .queue(Uuid::now_v7(), None, PageRequest::default())
            .await
            .unwrap();

        assert_eq!(queue.submissions.total, 0);
    }

    #[tokio::test]
    async fn approve_saves_submission_and_material_in_one_transaction() {
        let journal = Arc::new(Journal::default());

        let dto = service(users(true), submissions_with(pending()), journal.clone())
            .decide(Uuid::now_v7(), Uuid::now_v7(), "approve", None)
            .await
            .unwrap();

        assert_eq!(dto.status, SubmissionStatus::Approved);
        assert_eq!(
            *journal.0.lock().unwrap(),
            ["material:Lecture", "submission:approved", "commit"]
        );
    }

    #[tokio::test]
    async fn reject_requires_comment() {
        let mut submissions = submissions_with(pending());
        submissions.expect_update().never();

        let error = service(users(true), submissions, Default::default())
            .decide(Uuid::now_v7(), Uuid::now_v7(), "reject", None)
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::Validation(errors) if errors.contains_key("moderator_comment")));
    }

    #[tokio::test]
    async fn decide_on_reviewed_submission_is_validation_error() {
        let mut approved = pending();
        approved.approve(UserId::new(), Utc::now()).unwrap();

        let error = service(users(true), submissions_with(approved), Default::default())
            .decide(Uuid::now_v7(), Uuid::now_v7(), "reject", Some("late".into()))
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::Validation(errors) if errors.contains_key("status")));
    }

    #[tokio::test]
    async fn unknown_action_is_validation_error() {
        let error = service(users(true), submissions_with(pending()), Default::default())
            .decide(Uuid::now_v7(), Uuid::now_v7(), "publish", None)
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::Validation(errors) if errors.contains_key("action")));
    }
}
