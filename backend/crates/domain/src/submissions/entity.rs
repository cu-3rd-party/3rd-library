use chrono::{DateTime, Utc};

use super::SubmissionStatus;
use crate::DomainError;
use crate::common::{Audit, entity_id};
use crate::materials::{FileId, Material, MaterialFile, MaterialId, MaterialInfo};
use crate::users::UserId;

entity_id!(SubmissionId);

/// Правка заявки автором. `None` — поле не меняется; пустые строки и списки тоже игнорируются
/// (так форма шлёт незаполненные поля), кроме `description`, которое можно очистить.
#[derive(Debug, Clone, Default)]
pub struct SubmissionChanges {
    pub title: Option<String>,
    pub description: Option<String>,
    pub courses: Option<Vec<String>>,
    pub subjects: Option<Vec<String>>,
    pub kind: Option<String>,
    pub difficulty: Option<String>,
    /// Какие из уже прикреплённых файлов оставить; `None` — все.
    pub keep_file_ids: Option<Vec<FileId>>,
    /// Новые файлы (уже загружены в хранилище).
    pub new_files: Vec<MaterialFile>,
}

/// Агрегат «заявка на публикацию материала» вместе с прикреплёнными файлами.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Submission {
    id: SubmissionId,
    author: UserId,
    material_id: Option<MaterialId>,
    info: MaterialInfo,
    status: SubmissionStatus,
    moderator_id: Option<UserId>,
    moderator_comment: String,
    files: Vec<MaterialFile>,
    submitted_at: Option<DateTime<Utc>>,
    reviewed_at: Option<DateTime<Utc>>,
    published_at: Option<DateTime<Utc>>,
    audit: Audit,
}

/// Состояние заявки из хранилища — для [`Submission::restore`].
#[derive(Debug, Clone)]
pub struct SubmissionData {
    pub id: SubmissionId,
    pub author: UserId,
    pub material_id: Option<MaterialId>,
    pub info: MaterialInfo,
    pub status: SubmissionStatus,
    pub moderator_id: Option<UserId>,
    pub moderator_comment: String,
    pub files: Vec<MaterialFile>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub audit: Audit,
}

impl Submission {
    /// Новая заявка сразу уходит на модерацию.
    pub fn submit(author: UserId, info: MaterialInfo, files: Vec<MaterialFile>, now: DateTime<Utc>) -> Self {
        Self {
            id: SubmissionId::new(),
            author,
            material_id: None,
            info,
            status: SubmissionStatus::PendingReview,
            moderator_id: None,
            moderator_comment: String::new(),
            files,
            submitted_at: Some(now),
            reviewed_at: None,
            published_at: None,
            audit: Audit::new(),
        }
    }

    /// Восстановление из хранилища (без проверок и без изменения аудита).
    pub fn restore(data: SubmissionData) -> Self {
        Self {
            id: data.id,
            author: data.author,
            material_id: data.material_id,
            info: data.info,
            status: data.status,
            moderator_id: data.moderator_id,
            moderator_comment: data.moderator_comment,
            files: data.files,
            submitted_at: data.submitted_at,
            reviewed_at: data.reviewed_at,
            published_at: data.published_at,
            audit: data.audit,
        }
    }

    /// Обратно в состояние для хранилища (например, чтобы переупорядочить файлы).
    pub fn into_data(self) -> SubmissionData {
        SubmissionData {
            id: self.id,
            author: self.author,
            material_id: self.material_id,
            info: self.info,
            status: self.status,
            moderator_id: self.moderator_id,
            moderator_comment: self.moderator_comment,
            files: self.files,
            submitted_at: self.submitted_at,
            reviewed_at: self.reviewed_at,
            published_at: self.published_at,
            audit: self.audit,
        }
    }

    pub fn is_owned_by(&self, user: UserId) -> bool {
        self.author == user
    }

    /// Автор может править черновик и отклонённую заявку.
    pub fn is_editable(&self) -> bool {
        matches!(self.status, SubmissionStatus::Draft | SubmissionStatus::Rejected)
    }

    /// Правка автором. Если что-то изменилось, заявка снова уходит на модерацию.
    pub fn edit(&mut self, changes: SubmissionChanges, now: DateTime<Utc>) -> Result<(), DomainError> {
        if !self.is_editable() {
            return Err(DomainError::InvalidState(
                "Can only update draft or rejected submissions",
            ));
        }

        let mut changed = false;
        let mut set = |field: &mut String, value: Option<String>| {
            if let Some(value) = value.filter(|value| !value.is_empty()) {
                *field = value;
                changed = true;
            }
        };
        set(&mut self.info.title, changes.title);
        set(&mut self.info.kind, changes.kind);
        set(&mut self.info.difficulty, changes.difficulty);

        let mut set_list = |field: &mut Vec<String>, value: Option<Vec<String>>| {
            if let Some(value) = value.filter(|value| !value.is_empty()) {
                *field = value;
                changed = true;
            }
        };
        set_list(&mut self.info.courses, changes.courses);
        set_list(&mut self.info.subjects, changes.subjects);

        if let Some(description) = changes.description {
            self.info.description = description;
            changed = true;
        }

        if let Some(keep) = changes.keep_file_ids {
            self.files.retain(|file| keep.contains(&file.id));
        }
        if !changes.new_files.is_empty() {
            self.files.extend(changes.new_files);
            changed = true;
        }

        if changed {
            self.status = SubmissionStatus::PendingReview;
            self.submitted_at.get_or_insert(now);
            self.audit.touch();
        }
        Ok(())
    }

    /// Одобрение модератором: возвращает опубликованный материал с теми же данными и файлами.
    pub fn approve(&mut self, moderator: UserId, now: DateTime<Utc>) -> Result<Material, DomainError> {
        self.ensure_pending_review()?;

        let material = Material::publish(
            self.author,
            self.info.clone(),
            self.files.iter().map(|file| file.id).collect(),
            now,
        );
        self.status = SubmissionStatus::Approved;
        self.material_id = Some(material.id());
        self.moderator_comment.clear();
        self.review(moderator, now);
        self.published_at = Some(now);
        Ok(material)
    }

    /// Отклонение модератором с комментарием для автора.
    pub fn reject(&mut self, moderator: UserId, comment: String, now: DateTime<Utc>) -> Result<(), DomainError> {
        self.ensure_pending_review()?;

        self.status = SubmissionStatus::Rejected;
        self.moderator_comment = comment;
        self.review(moderator, now);
        Ok(())
    }

    fn ensure_pending_review(&self) -> Result<(), DomainError> {
        if self.status != SubmissionStatus::PendingReview {
            return Err(DomainError::InvalidState(
                "Can only moderate pending_review submissions",
            ));
        }
        Ok(())
    }

    fn review(&mut self, moderator: UserId, now: DateTime<Utc>) {
        self.moderator_id = Some(moderator);
        self.reviewed_at = Some(now);
        self.audit.touch();
    }

    pub fn id(&self) -> SubmissionId {
        self.id
    }

    pub fn author(&self) -> UserId {
        self.author
    }

    pub fn material_id(&self) -> Option<MaterialId> {
        self.material_id
    }

    pub fn info(&self) -> &MaterialInfo {
        &self.info
    }

    pub fn status(&self) -> SubmissionStatus {
        self.status
    }

    pub fn moderator_id(&self) -> Option<UserId> {
        self.moderator_id
    }

    pub fn moderator_comment(&self) -> &str {
        &self.moderator_comment
    }

    pub fn files(&self) -> &[MaterialFile] {
        &self.files
    }

    pub fn submitted_at(&self) -> Option<DateTime<Utc>> {
        self.submitted_at
    }

    pub fn reviewed_at(&self) -> Option<DateTime<Utc>> {
        self.reviewed_at
    }

    pub fn published_at(&self) -> Option<DateTime<Utc>> {
        self.published_at
    }

    pub fn audit(&self) -> &Audit {
        &self.audit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::materials::FileName;

    fn info() -> MaterialInfo {
        MaterialInfo {
            title: "Lecture".into(),
            description: "Notes".into(),
            courses: vec!["1".into()],
            subjects: vec!["math".into()],
            kind: "notes".into(),
            difficulty: "none".into(),
        }
    }

    fn file(author: UserId) -> MaterialFile {
        MaterialFile::new(author, FileName::parse("a.pdf").unwrap(), 10, None)
    }

    fn rejected(author: UserId) -> Submission {
        let mut submission = Submission::submit(author, info(), vec![file(author)], Utc::now());
        submission.reject(UserId::new(), "fix it".into(), Utc::now()).unwrap();
        submission
    }

    #[test]
    fn submit_is_pending_review() {
        let submission = Submission::submit(UserId::new(), info(), vec![], Utc::now());

        assert_eq!(submission.status(), SubmissionStatus::PendingReview);
        assert!(submission.submitted_at().is_some());
        assert!(!submission.is_editable());
    }

    #[test]
    fn edit_pending_review_is_rejected() {
        let mut submission = Submission::submit(UserId::new(), info(), vec![], Utc::now());

        let error = submission.edit(SubmissionChanges::default(), Utc::now()).unwrap_err();

        assert!(matches!(error, DomainError::InvalidState(_)));
    }

    #[test]
    fn edit_ignores_empty_values_and_resubmits() {
        let author = UserId::new();
        let mut submission = rejected(author);

        submission
            .edit(
                SubmissionChanges {
                    title: Some(String::new()),
                    courses: Some(vec![]),
                    kind: Some("slides".into()),
                    ..Default::default()
                },
                Utc::now(),
            )
            .unwrap();

        assert_eq!(submission.info().title, "Lecture");
        assert_eq!(submission.info().courses, ["1"]);
        assert_eq!(submission.info().kind, "slides");
        assert_eq!(submission.status(), SubmissionStatus::PendingReview);
    }

    #[test]
    fn edit_without_changes_keeps_status() {
        let mut submission = rejected(UserId::new());

        submission.edit(SubmissionChanges::default(), Utc::now()).unwrap();

        assert_eq!(submission.status(), SubmissionStatus::Rejected);
    }

    #[test]
    fn edit_keeps_new_files_alongside_kept_ones() {
        let author = UserId::new();
        let mut submission = rejected(author);
        let old = submission.files()[0].id;
        let new = file(author);
        let new_id = new.id;

        submission
            .edit(
                SubmissionChanges {
                    keep_file_ids: Some(vec![old]),
                    new_files: vec![new],
                    ..Default::default()
                },
                Utc::now(),
            )
            .unwrap();

        let ids: Vec<_> = submission.files().iter().map(|file| file.id).collect();
        assert_eq!(ids, [old, new_id]);
    }

    #[test]
    fn edit_drops_files_not_kept() {
        let mut submission = rejected(UserId::new());

        submission
            .edit(
                SubmissionChanges {
                    keep_file_ids: Some(vec![]),
                    ..Default::default()
                },
                Utc::now(),
            )
            .unwrap();

        assert!(submission.files().is_empty());
    }

    #[test]
    fn approve_publishes_material_with_files() {
        let author = UserId::new();
        let moderator = UserId::new();
        let mut submission = Submission::submit(author, info(), vec![file(author)], Utc::now());

        let material = submission.approve(moderator, Utc::now()).unwrap();

        assert_eq!(submission.status(), SubmissionStatus::Approved);
        assert_eq!(submission.material_id(), Some(material.id()));
        assert_eq!(submission.moderator_id(), Some(moderator));
        assert_eq!(material.author(), author);
        assert_eq!(material.file_ids(), [submission.files()[0].id]);
        assert!(submission.approve(moderator, Utc::now()).is_err());
    }

    #[test]
    fn reject_requires_pending_review() {
        let mut submission = rejected(UserId::new());

        assert!(submission.reject(UserId::new(), "again".into(), Utc::now()).is_err());
    }
}
