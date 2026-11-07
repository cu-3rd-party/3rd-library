//! Репозитории, единица работы и хранилище сессий поверх [`Store`]: чтения — из памяти,
//! записи — через групповой коммит (ответ после коммита).

use std::sync::{Arc, Mutex, PoisonError};

use async_trait::async_trait;
use chrono::TimeDelta;
use library_application::auth::Session;
use library_application::ports::{ExternalError, ExternalResult, SessionStore};
use library_domain::RepositoryError;
use library_domain::common::{Page, PageRequest, now};
use library_domain::error::RepositoryResult;
use library_domain::materials::{Material, MaterialFile, MaterialId, MaterialRepository, MaterialSummary};
use library_domain::submissions::{StatusCounters, Submission, SubmissionId, SubmissionRepository, SubmissionStatus};
use library_domain::unit_of_work::{Transaction, UnitOfWork};
use library_domain::users::{AuthorSummary, User, UserId, UserRepository};
use uuid::Uuid;

use super::like::{Pattern, lowercase};
use super::store::{Change, MaterialRec, Store, Users, status_slot};
use super::writer::is_unique_violation;
use crate::metrics::record_user_created;

/// Изменения открытой транзакции: коммитятся вместе в [`Transaction::commit`].
type Pending = Mutex<Vec<Change>>;

/// Куда пишет репозиторий: сразу (своя транзакция на запись) или в открытую транзакцию.
#[derive(Clone)]
struct Target {
    store: Arc<Store>,
    transaction: Option<Arc<Pending>>,
}

impl Target {
    async fn write(&self, change: Change) -> RepositoryResult<()> {
        match &self.transaction {
            Some(pending) => {
                pending.lock().unwrap_or_else(PoisonError::into_inner).push(change);
                Ok(())
            }
            None => self.store.commit(vec![change]).await.map_err(RepositoryError::new),
        }
    }
}

/// `offset`/`limit` страницы для срезов в памяти.
fn window(page: PageRequest) -> (usize, usize) {
    let convert = |value: i64| usize::try_from(value).unwrap_or(usize::MAX);
    (convert(page.offset()), convert(page.limit()))
}

fn total(len: usize) -> i64 {
    i64::try_from(len).unwrap_or(i64::MAX)
}

// ---------------------------------------------------------------- users

pub struct MemoryUserRepository {
    target: Target,
}

impl MemoryUserRepository {
    pub fn new(store: Arc<Store>) -> Self {
        Self {
            target: Target {
                store,
                transaction: None,
            },
        }
    }
}

#[async_trait]
impl UserRepository for MemoryUserRepository {
    async fn find_by_id(&self, id: UserId) -> RepositoryResult<Option<User>> {
        Ok(self.target.store.users().get(id.into_inner()).cloned())
    }

    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>> {
        let users = self.target.store.users();
        Ok(users.by_email.get(email).and_then(|id| users.get(*id)).cloned())
    }

    async fn insert(&self, user: &User) -> RepositoryResult<bool> {
        let email = user.email().as_str().to_owned();
        let taken = |users: &Users| users.by_email.contains_key(&email) || users.reserved_emails.contains(&email);

        if self.target.transaction.is_some() {
            if taken(&self.target.store.users()) {
                return Ok(false);
            }
            self.target.write(Change::InsertUser(user.clone())).await?;
            return Ok(true);
        }

        {
            let mut users = self.target.store.users_mut();
            if taken(&users) {
                return Ok(false);
            }
            users.reserved_emails.insert(email.clone());
        }
        let result = self.target.store.commit(vec![Change::InsertUser(user.clone())]).await;
        self.target.store.users_mut().reserved_emails.remove(&email);
        match result {
            Ok(()) => {
                record_user_created();
                Ok(true)
            }
            // адрес занят в обход бэкенда — для клиента это тот же конфликт
            Err(error) if is_unique_violation(&error) => Ok(false),
            Err(error) => Err(RepositoryError::new(error)),
        }
    }

    async fn update(&self, user: &User) -> RepositoryResult<bool> {
        if self.target.store.users().get(user.id().into_inner()).is_none() {
            return Ok(false);
        }
        self.target.write(Change::UpdateUser(user.clone())).await?;
        Ok(true)
    }

    async fn list_authors(&self, search: Option<String>, page: PageRequest) -> RepositoryResult<Page<AuthorSummary>> {
        let pattern = search.map(|search| Pattern::contains(&search));
        let users = self.target.store.users();
        let catalog = self.target.store.catalog();

        let matching = catalog.authors.iter().filter_map(|id| users.get(*id)).filter(|user| {
            pattern
                .as_ref()
                .is_none_or(|pattern| pattern.matches(&lowercase(user.name())))
        });
        let (offset, limit) = window(page);
        let mut count = 0;
        let mut items = Vec::new();
        for user in matching {
            if count >= offset && items.len() < limit {
                let id = user.id().into_inner();
                items.push(AuthorSummary {
                    id: user.id(),
                    name: user.name().to_owned(),
                    bio: user.bio().to_owned(),
                    is_email_verified: user.is_email_verified(),
                    materials_count: total(catalog.by_author.get(&id).map_or(0, Vec::len)),
                });
            }
            count += 1;
        }
        Ok(Page {
            items,
            total: total(count),
        })
    }
}

// ---------------------------------------------------------------- materials

pub struct MemoryMaterialRepository {
    target: Target,
}

impl MemoryMaterialRepository {
    pub fn new(store: Arc<Store>) -> Self {
        Self {
            target: Target {
                store,
                transaction: None,
            },
        }
    }
}

fn summary(material: &MaterialRec, users: &Users) -> MaterialSummary {
    MaterialSummary {
        id: material.id.into(),
        author_id: material.author.into(),
        author_name: users
            .get(material.author)
            .map(|user| user.name().to_owned())
            .unwrap_or_default(),
        info: material.info.clone(),
        published_at: material.published_at,
        created: material.created,
    }
}

fn materials_page(list: &[Arc<MaterialRec>], page: PageRequest, users: &Users) -> Page<MaterialSummary> {
    let (offset, limit) = window(page);
    Page {
        items: list
            .iter()
            .skip(offset)
            .take(limit)
            .map(|material| summary(material, users))
            .collect(),
        total: total(list.len()),
    }
}

#[async_trait]
impl MaterialRepository for MemoryMaterialRepository {
    async fn list_published(
        &self,
        search: Option<String>,
        page: PageRequest,
    ) -> RepositoryResult<Page<MaterialSummary>> {
        let users = self.target.store.users();
        let catalog = self.target.store.catalog();
        Ok(match search {
            Some(search) => materials_page(&catalog.search(&format!("%{search}%")), page, &users),
            None => materials_page(&catalog.feed, page, &users),
        })
    }

    async fn list_published_by_author(
        &self,
        author: UserId,
        page: PageRequest,
    ) -> RepositoryResult<Page<MaterialSummary>> {
        let users = self.target.store.users();
        let catalog = self.target.store.catalog();
        let list = catalog.by_author.get(&author.into_inner()).map_or(&[][..], Vec::as_slice);
        Ok(materials_page(list, page, &users))
    }

    async fn find_by_id(&self, id: MaterialId) -> RepositoryResult<Option<MaterialSummary>> {
        let users = self.target.store.users();
        let catalog = self.target.store.catalog();
        Ok(catalog
            .by_id
            .get(&id.into_inner())
            .map(|material| summary(material, &users)))
    }

    async fn files(&self, id: MaterialId) -> RepositoryResult<Vec<MaterialFile>> {
        let catalog = self.target.store.catalog();
        Ok(catalog
            .by_id
            .get(&id.into_inner())
            .map(|material| material.files.clone())
            .unwrap_or_default())
    }

    async fn insert(&self, material: &Material) -> RepositoryResult<()> {
        self.target.write(Change::InsertMaterial(material.clone())).await
    }
}

// ---------------------------------------------------------------- submissions

pub struct MemorySubmissionRepository {
    target: Target,
}

impl MemorySubmissionRepository {
    pub fn new(store: Arc<Store>) -> Self {
        Self {
            target: Target {
                store,
                transaction: None,
            },
        }
    }
}

fn submissions_page<'a>(
    ids: impl Iterator<Item = &'a Uuid>,
    len: usize,
    page: PageRequest,
    find: impl Fn(&Uuid) -> Option<Submission>,
) -> Page<Submission> {
    let (offset, limit) = window(page);
    Page {
        items: ids.skip(offset).take(limit).filter_map(find).collect(),
        total: total(len),
    }
}

#[async_trait]
impl SubmissionRepository for MemorySubmissionRepository {
    async fn find_by_id(&self, id: SubmissionId) -> RepositoryResult<Option<Submission>> {
        let submissions = self.target.store.submissions();
        Ok(submissions
            .by_id
            .get(&id.into_inner())
            .map(|submission| (**submission).clone()))
    }

    async fn list_by_author(
        &self,
        author: UserId,
        status: Option<SubmissionStatus>,
        page: PageRequest,
    ) -> RepositoryResult<Page<Submission>> {
        let submissions = self.target.store.submissions();
        let find = |id: &Uuid| submissions.by_id.get(id).map(|submission| (**submission).clone());
        Ok(match submissions.by_author.get(&author.into_inner()) {
            Some(own) => {
                let set = &own[status_slot(status)];
                submissions_page(set.iter().map(|(_, id)| id), set.len(), page, find)
            }
            None => submissions_page(std::iter::empty(), 0, page, find),
        })
    }

    async fn list_for_moderation(
        &self,
        status: Option<SubmissionStatus>,
        page: PageRequest,
    ) -> RepositoryResult<Page<Submission>> {
        let submissions = self.target.store.submissions();
        let set = &submissions.queue[status_slot(status)];
        let find = |id: &Uuid| submissions.by_id.get(id).map(|submission| (**submission).clone());
        Ok(submissions_page(set.iter().map(|(_, id)| id), set.len(), page, find))
    }

    async fn count_by_status(&self) -> RepositoryResult<StatusCounters> {
        let submissions = self.target.store.submissions();
        let count = |status| total(submissions.queue[status_slot(status)].len());
        Ok(StatusCounters {
            all: count(None),
            draft: count(Some(SubmissionStatus::Draft)),
            pending_review: count(Some(SubmissionStatus::PendingReview)),
            rejected: count(Some(SubmissionStatus::Rejected)),
            approved: count(Some(SubmissionStatus::Approved)),
        })
    }

    async fn insert(&self, submission: &Submission) -> RepositoryResult<()> {
        self.target.write(Change::InsertSubmission(submission.clone())).await
    }

    async fn update(&self, submission: &Submission) -> RepositoryResult<bool> {
        if !self
            .target
            .store
            .submissions()
            .by_id
            .contains_key(&submission.id().into_inner())
        {
            return Ok(false);
        }
        self.target.write(Change::UpdateSubmission(submission.clone())).await?;
        Ok(true)
    }
}

// ---------------------------------------------------------------- unit of work

pub struct MemoryUnitOfWork {
    store: Arc<Store>,
}

impl MemoryUnitOfWork {
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }
}

#[async_trait]
impl UnitOfWork for MemoryUnitOfWork {
    async fn begin(&self) -> RepositoryResult<Box<dyn Transaction>> {
        Ok(Box::new(MemoryTransaction {
            target: Target {
                store: self.store.clone(),
                transaction: Some(Arc::default()),
            },
        }))
    }
}

/// Записи копятся и уходят в БД одной транзакцией при `commit`; без него просто отбрасываются.
pub struct MemoryTransaction {
    target: Target,
}

#[async_trait]
impl Transaction for MemoryTransaction {
    fn users(&self) -> Box<dyn UserRepository + '_> {
        Box::new(MemoryUserRepository {
            target: self.target.clone(),
        })
    }

    fn materials(&self) -> Box<dyn MaterialRepository + '_> {
        Box::new(MemoryMaterialRepository {
            target: self.target.clone(),
        })
    }

    fn submissions(&self) -> Box<dyn SubmissionRepository + '_> {
        Box::new(MemorySubmissionRepository {
            target: self.target.clone(),
        })
    }

    async fn commit(self: Box<Self>) -> RepositoryResult<()> {
        let pending = self.target.transaction.as_ref().expect("transaction target");
        let changes = std::mem::take(&mut *pending.lock().unwrap_or_else(PoisonError::into_inner));
        self.target.store.commit(changes).await.map_err(RepositoryError::new)
    }

    async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
        Ok(())
    }
}

// ---------------------------------------------------------------- sessions

/// Сессии: в памяти и в таблице `session`; просроченные не находятся.
pub struct MemorySessionStore {
    store: Arc<Store>,
}

impl MemorySessionStore {
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }
}

#[async_trait]
impl SessionStore for MemorySessionStore {
    async fn save(&self, session: &Session, ttl: TimeDelta) -> ExternalResult<()> {
        self.store
            .commit(vec![Change::SaveSession(*session, now() + ttl)])
            .await
            .map_err(ExternalError::new)
    }

    async fn find(&self, session_id: Uuid) -> ExternalResult<Option<Session>> {
        let now = now();
        Ok(self
            .store
            .sessions()
            .get(&session_id)
            .filter(|session| session.expires_at > now)
            .map(|session| Session {
                user_id: session.user_id,
                session_id,
                verified: session.verified,
            }))
    }

    async fn delete(&self, session_id: Uuid) -> ExternalResult<()> {
        if !self.store.sessions().contains_key(&session_id) {
            return Ok(());
        }
        self.store
            .commit(vec![Change::DeleteSession(session_id)])
            .await
            .map_err(ExternalError::new)
    }
}
