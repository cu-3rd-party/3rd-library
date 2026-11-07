//! Состояние в памяти процесса: пользователи, сессии, каталог материалов, заявки.
//!
//! Допущение: бэкенд запущен в одном экземпляре и все записи идут через него. Исключение — авторизация:
//! роли выдают прямо в БД, а сессию можно отозвать удалением строки. Поэтому раз в
//! [`AUTH_SYNC_INTERVAL`] роли и сессии сверяются с БД ([`Store::sync_auth`]).
//! Состояние загружается целиком при старте, чтения не ходят в БД.
//!
//! Запись: изменения сначала коммитятся в БД (групповой коммит, [`Writer`]), и только потом
//! применяются к памяти. Поэтому читатели видят только закоммиченное, а успешный ответ значит
//! «записано в БД». Сценарии «прочитать → изменить → записать» сериализуются замками приложения
//! по id сущности.
//!
//! Порядок взятия замков (во избежание взаимоблокировок): users → catalog → submissions → sessions.

use std::cmp::Reverse;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::{Arc, Mutex, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::Duration;

use chrono::{DateTime, Utc};
use library_application::auth::Session;
use library_domain::common::now;
use library_domain::materials::{Material, MaterialFile, MaterialInfo};
use library_domain::submissions::{Submission, SubmissionStatus};
use library_domain::users::{ROLE_USER, User};
use sqlx::PgPool;
use uuid::Uuid;

use super::like::{Pattern, lowercase};
use super::rows::{
    FILE_COLUMNS, FileRow, LinkedFileRow, MATERIAL_COLUMNS, MaterialRow, SUBMISSION_COLUMNS, SubmissionRow,
    USER_COLUMNS, UserRow,
};
use super::writer::{self, Batch, WriteError, Writer};
use crate::metrics::observe_db_query;

const SESSION_CLEANUP_INTERVAL: Duration = Duration::from_secs(600);
/// Как часто сверять роли и сессии с БД (изменения в обход бэкенда).
pub const AUTH_SYNC_INTERVAL: Duration = Duration::from_secs(60);
/// Пределы кеша поиска: число шаблонов, суммарное число ссылок и размер одного результата.
const SEARCH_CACHE_PATTERNS: usize = 1024;
const SEARCH_CACHE_ITEMS: usize = 1_000_000;
const SEARCH_CACHE_MAX_RESULT: usize = 50_000;

fn read<T>(lock: &RwLock<T>) -> RwLockReadGuard<'_, T> {
    lock.read().unwrap_or_else(PoisonError::into_inner)
}

fn write<T>(lock: &RwLock<T>) -> RwLockWriteGuard<'_, T> {
    lock.write().unwrap_or_else(PoisonError::into_inner)
}

// ---------------------------------------------------------------- users

#[derive(Default)]
pub(crate) struct Users {
    pub(crate) by_id: HashMap<Uuid, Arc<User>>,
    pub(crate) by_email: HashMap<String, Uuid>,
    /// Адреса регистраций, которые сейчас коммитятся (уникальность email без гонок).
    pub(crate) reserved_emails: HashSet<String>,
}

impl Users {
    pub(crate) fn get(&self, id: Uuid) -> Option<&User> {
        self.by_id.get(&id).map(AsRef::as_ref)
    }

    fn put(&mut self, user: User) {
        self.by_email.insert(user.email().as_str().to_owned(), user.id().into_inner());
        self.by_id.insert(user.id().into_inner(), Arc::new(user));
    }
}

// ---------------------------------------------------------------- catalog

/// Материал каталога.
pub(crate) struct MaterialRec {
    pub(crate) id: Uuid,
    pub(crate) author: Uuid,
    pub(crate) info: MaterialInfo,
    title_lower: String,
    pub(crate) published_at: Option<DateTime<Utc>>,
    pub(crate) created: DateTime<Utc>,
    /// В порядке `created_at, file_id`.
    pub(crate) files: Vec<MaterialFile>,
}

type FeedKey = (Reverse<DateTime<Utc>>, Uuid);

impl MaterialRec {
    fn new(
        id: Uuid,
        author: Uuid,
        info: MaterialInfo,
        published_at: Option<DateTime<Utc>>,
        created: DateTime<Utc>,
        files: Vec<MaterialFile>,
    ) -> Self {
        Self {
            title_lower: lowercase(&info.title),
            id,
            author,
            info,
            published_at,
            created,
            files,
        }
    }

    /// Лента: `published_at desc, material_id`.
    fn feed_key(&self) -> FeedKey {
        (Reverse(self.published_at.unwrap_or_default()), self.id)
    }
}

type Materials = Arc<Vec<Arc<MaterialRec>>>;

#[derive(Default)]
struct SearchCache {
    entries: HashMap<String, (Pattern, Materials)>,
    items: usize,
}

#[derive(Default)]
pub(crate) struct Catalog {
    pub(crate) by_id: HashMap<Uuid, Arc<MaterialRec>>,
    /// Опубликованные, в порядке ленты.
    pub(crate) feed: Vec<Arc<MaterialRec>>,
    /// Опубликованные материалы автора, в порядке ленты. Ключи — ровно авторы.
    pub(crate) by_author: HashMap<Uuid, Vec<Arc<MaterialRec>>>,
    /// Авторы в порядке `name, user_id` по правилам сортировки БД.
    pub(crate) authors: Vec<Uuid>,
    /// Результаты поиска по названию: шаблон ILIKE → материалы в порядке ленты.
    /// Новые материалы дописываются в уже найденные результаты, поэтому кеш всегда точен.
    search: Mutex<SearchCache>,
}

impl Catalog {
    /// Опубликованные материалы с названием `ILIKE pattern`, в порядке ленты.
    pub(crate) fn search(&self, pattern: &str) -> Materials {
        let key = lowercase(pattern);
        if let Some((_, hit)) = self.cache().entries.get(&key) {
            return hit.clone();
        }
        let compiled = Pattern::new(pattern);
        let found: Materials = Arc::new(
            self.feed
                .iter()
                .filter(|material| compiled.matches(&material.title_lower))
                .cloned()
                .collect(),
        );
        // каталог не меняется, пока держим его на чтение, — результат точен на момент вставки
        if found.len() <= SEARCH_CACHE_MAX_RESULT {
            let mut cache = self.cache();
            if cache.entries.len() >= SEARCH_CACHE_PATTERNS || cache.items + found.len() > SEARCH_CACHE_ITEMS {
                *cache = SearchCache::default();
            }
            cache.items += found.len();
            cache.entries.insert(key, (compiled, found.clone()));
        }
        found
    }

    fn cache(&self) -> std::sync::MutexGuard<'_, SearchCache> {
        self.search.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn insert_sorted(list: &mut Vec<Arc<MaterialRec>>, material: &Arc<MaterialRec>) {
        let key = material.feed_key();
        let position = list.partition_point(|other| other.feed_key() < key);
        list.insert(position, material.clone());
    }

    fn publish(&mut self, material: MaterialRec) {
        let material = Arc::new(material);
        self.by_id.insert(material.id, material.clone());
        if material.published_at.is_none() {
            return;
        }
        Self::insert_sorted(&mut self.feed, &material);
        Self::insert_sorted(self.by_author.entry(material.author).or_default(), &material);

        let cache = self.search.get_mut().unwrap_or_else(PoisonError::into_inner);
        for (pattern, found) in cache.entries.values_mut() {
            if pattern.matches(&material.title_lower) {
                Self::insert_sorted(Arc::make_mut(found), &material);
                cache.items += 1;
            }
        }
    }
}

// ---------------------------------------------------------------- submissions

/// Очередь модерации: старые сверху.
type QueueKey = (DateTime<Utc>, Uuid);
/// Заявки автора: новые сверху, при равном времени — по id.
type OwnKey = (Reverse<DateTime<Utc>>, Uuid);

/// Индекс в массивах по статусам; последний — все статусы.
const ANY_STATUS: usize = SubmissionStatus::ALL.len();

pub(crate) fn status_slot(status: Option<SubmissionStatus>) -> usize {
    status.map_or(ANY_STATUS, |status| {
        SubmissionStatus::ALL
            .iter()
            .position(|known| *known == status)
            .expect("status is in ALL")
    })
}

#[derive(Default)]
pub(crate) struct Submissions {
    pub(crate) by_id: HashMap<Uuid, Arc<Submission>>,
    pub(crate) queue: [BTreeSet<QueueKey>; ANY_STATUS + 1],
    pub(crate) by_author: HashMap<Uuid, [BTreeSet<OwnKey>; ANY_STATUS + 1]>,
}

impl Submissions {
    fn put(&mut self, submission: Submission) {
        let id = submission.id().into_inner();
        let created = submission.audit().created();
        let own = self.by_author.entry(submission.author().into_inner()).or_default();
        if let Some(old) = self.by_id.get(&id) {
            let slot = status_slot(Some(old.status()));
            self.queue[slot].remove(&(old.audit().created(), id));
            own[slot].remove(&(Reverse(old.audit().created()), id));
        }
        for slot in [status_slot(Some(submission.status())), ANY_STATUS] {
            self.queue[slot].insert((created, id));
            own[slot].insert((Reverse(created), id));
        }
        self.by_id.insert(id, Arc::new(submission));
    }
}

// ---------------------------------------------------------------- sessions

#[derive(Clone, Copy)]
pub(crate) struct SessionRec {
    pub(crate) user_id: Uuid,
    pub(crate) verified: bool,
    pub(crate) expires_at: DateTime<Utc>,
}

// ---------------------------------------------------------------- changes

/// Изменение, которое коммитится в БД и затем применяется к памяти.
pub(crate) enum Change {
    InsertUser(User),
    UpdateUser(User),
    SaveSession(Session, DateTime<Utc>),
    DeleteSession(Uuid),
    InsertSubmission(Submission),
    UpdateSubmission(Submission),
    /// Файлы материала уже сохранены вместе с заявкой; сохраняются только связи.
    InsertMaterial(Material),
}

fn batch_of(changes: &[Change]) -> Batch {
    let mut batch = Batch::default();
    for change in changes {
        match change {
            Change::InsertUser(user) | Change::UpdateUser(user) => batch.users.push(writer::UserRow {
                user_id: user.id().into_inner(),
                email: user.email().as_str().to_owned(),
                password_hash: user.password_hash().as_str().to_owned(),
                name: user.name().to_owned(),
                bio: user.bio().to_owned(),
                is_email_verified: user.is_email_verified(),
                verification_code: user.verification_code().map(|code| code.as_str().to_owned()),
                verification_code_issued_at: user.code_issued_at(),
                verification_code_expires_at: user.code_expires_at(),
                created_at: user.audit().created(),
                updated_at: user.audit().last_modified(),
            }),
            Change::SaveSession(session, expires_at) => batch.sessions.push(writer::SessionRow {
                session_id: session.session_id,
                user_id: session.user_id,
                verified: session.verified,
                expires_at: *expires_at,
            }),
            Change::DeleteSession(id) => batch.deleted_sessions.push(*id),
            Change::InsertSubmission(submission) | Change::UpdateSubmission(submission) => {
                let info = submission.info();
                batch.submissions.push(writer::SubmissionRow {
                    submission_id: submission.id().into_inner(),
                    user_id: submission.author().into_inner(),
                    material_id: submission.material_id().map(|id| id.into_inner()),
                    title: info.title.clone(),
                    description: info.description.clone(),
                    courses: info.courses.clone(),
                    subjects: info.subjects.clone(),
                    r#type: info.kind.clone(),
                    difficulty: info.difficulty.clone(),
                    status: submission.status().as_str(),
                    moderator_id: submission.moderator_id().map(|id| id.into_inner()),
                    moderator_comment: submission.moderator_comment().to_owned(),
                    published_at: submission.published_at(),
                    created_at: submission.audit().created(),
                    updated_at: submission.audit().last_modified(),
                    submitted_at: submission.submitted_at(),
                    reviewed_at: submission.reviewed_at(),
                });
                batch.files.extend(submission.files().iter().map(|file| writer::FileRow {
                    file_id: file.id.into_inner(),
                    user_id: file.owner.into_inner(),
                    name: file.name.clone(),
                    size_bytes: file.size_bytes,
                    extension: file.extension.clone(),
                    mime_type: file.mime_type.clone(),
                    storage_key: file.storage_key.clone(),
                    created_at: file.created,
                }));
                batch.submission_files.push(writer::SubmissionFilesRow {
                    submission_id: submission.id().into_inner(),
                    file_ids: submission.files().iter().map(|file| file.id.into_inner()).collect(),
                });
            }
            Change::InsertMaterial(material) => {
                let info = material.info();
                batch.materials.push(writer::MaterialRow {
                    material_id: material.id().into_inner(),
                    user_id: material.author().into_inner(),
                    title: info.title.clone(),
                    description: info.description.clone(),
                    courses: info.courses.clone(),
                    subjects: info.subjects.clone(),
                    r#type: info.kind.clone(),
                    difficulty: info.difficulty.clone(),
                    published_at: material.published_at(),
                    created_at: material.audit().created(),
                    updated_at: material.audit().last_modified(),
                });
                batch
                    .material_files
                    .extend(material.file_ids().iter().map(|file_id| writer::MaterialFileRow {
                        material_id: material.id().into_inner(),
                        file_id: file_id.into_inner(),
                    }));
            }
        }
    }
    batch
}

fn sort_files(files: &mut [MaterialFile]) {
    files.sort_by_key(|file| (file.created, file.id.into_inner()));
}

// ---------------------------------------------------------------- store

pub struct Store {
    pub(crate) db: PgPool,
    writer: Writer,
    users: RwLock<Users>,
    catalog: RwLock<Catalog>,
    submissions: RwLock<Submissions>,
    sessions: RwLock<HashMap<Uuid, SessionRec>>,
    /// Пересчёт порядка авторов и его применение — по одному (см. [`Store::apply`]).
    authors_order: tokio::sync::Mutex<()>,
}

impl Store {
    /// Загружает всё состояние из БД и запускает запись и фоновую уборку сессий.
    pub async fn load(db: PgPool) -> Result<Arc<Self>, sqlx::Error> {
        let started = std::time::Instant::now();

        let mut users = Users::default();
        let rows: Vec<UserRow> = sqlx::query_as(&format!("select {USER_COLUMNS} from web_user"))
            .fetch_all(&db)
            .await?;
        for row in rows {
            users.put(row.into());
        }

        let mut files = linked_files(
            &db,
            "select mfr.material_id as owner_id, {FILE_COLUMNS} from material_file mf \
             inner join material_file_rel mfr on mf.file_id = mfr.file_id order by mf.created_at, mf.file_id",
        )
        .await?;
        let rows: Vec<MaterialRow> = sqlx::query_as(&format!("select {MATERIAL_COLUMNS} from material m"))
            .fetch_all(&db)
            .await?;
        let mut catalog = Catalog::default();
        for row in rows {
            let material = Arc::new(MaterialRec::new(
                row.material_id,
                row.user_id,
                row.info.into(),
                row.published_at,
                row.created_at.unwrap_or_default(),
                files.remove(&row.material_id).unwrap_or_default(),
            ));
            catalog.by_id.insert(material.id, material.clone());
            if material.published_at.is_some() {
                catalog.feed.push(material);
            }
        }
        catalog.feed.sort_by_key(|material| material.feed_key());
        for material in &catalog.feed {
            catalog.by_author.entry(material.author).or_default().push(material.clone());
        }
        catalog.authors = sqlx::query_scalar(
            "select u.user_id from web_user u where exists \
             (select 1 from material m where m.user_id = u.user_id and m.published_at is not null) \
             order by u.name, u.user_id",
        )
        .fetch_all(&db)
        .await?;

        let mut files = linked_files(
            &db,
            "select sfr.submission_id as owner_id, {FILE_COLUMNS} from material_file mf \
             inner join submission_file_rel sfr on mf.file_id = sfr.file_id order by mf.created_at, mf.file_id",
        )
        .await?;
        let rows: Vec<SubmissionRow> = sqlx::query_as(&format!("select {SUBMISSION_COLUMNS} from submission s"))
            .fetch_all(&db)
            .await?;
        let mut submissions = Submissions::default();
        for row in rows {
            let files = files.remove(&row.submission_id).unwrap_or_default();
            match row.into_domain(files) {
                Ok(submission) => submissions.put(submission),
                Err(error) => tracing::error!(%error, "skipping unreadable submission"),
            }
        }

        let rows: Vec<(Uuid, Uuid, bool, DateTime<Utc>)> =
            sqlx::query_as("select session_id, user_id, verified, expires_at from session where expires_at > now()")
                .fetch_all(&db)
                .await?;
        let sessions = rows
            .into_iter()
            .map(|(id, user_id, verified, expires_at)| {
                (
                    id,
                    SessionRec {
                        user_id,
                        verified,
                        expires_at,
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        tracing::info!(
            users = users.by_id.len(),
            materials = catalog.by_id.len(),
            submissions = submissions.by_id.len(),
            sessions = sessions.len(),
            elapsed_ms = started.elapsed().as_millis() as u64,
            "loaded state into memory"
        );

        let store = Arc::new(Self {
            writer: Writer::start(db.clone()),
            db,
            users: RwLock::new(users),
            catalog: RwLock::new(catalog),
            submissions: RwLock::new(submissions),
            sessions: RwLock::new(sessions),
            authors_order: tokio::sync::Mutex::new(()),
        });
        tokio::spawn(clean_sessions(Arc::downgrade(&store)));
        tokio::spawn(sync_auth_periodically(Arc::downgrade(&store)));
        Ok(store)
    }

    pub(crate) fn users(&self) -> RwLockReadGuard<'_, Users> {
        read(&self.users)
    }

    pub(crate) fn users_mut(&self) -> RwLockWriteGuard<'_, Users> {
        write(&self.users)
    }

    pub(crate) fn catalog(&self) -> RwLockReadGuard<'_, Catalog> {
        read(&self.catalog)
    }

    pub(crate) fn submissions(&self) -> RwLockReadGuard<'_, Submissions> {
        read(&self.submissions)
    }

    pub(crate) fn sessions(&self) -> RwLockReadGuard<'_, HashMap<Uuid, SessionRec>> {
        read(&self.sessions)
    }

    /// Коммитит изменения в БД (одной транзакцией) и после коммита применяет их к памяти.
    pub(crate) async fn commit(&self, changes: Vec<Change>) -> Result<(), WriteError> {
        if changes.is_empty() {
            return Ok(());
        }
        self.writer.write(batch_of(&changes)).await?;
        self.apply(changes).await;
        Ok(())
    }

    async fn apply(&self, changes: Vec<Change>) {
        let materials = self.material_records(&changes).await;

        // Новый автор или новое имя автора меняют порядок списка авторов. Он считается в БД
        // (её правила сортировки) уже после коммита и применяется вместе с изменениями под общим
        // замком: последний применённый порядок всегда посчитан после всех предыдущих коммитов.
        let changes_authors = {
            let users = self.users();
            let catalog = self.catalog();
            let new_author = materials
                .iter()
                .any(|material| material.published_at.is_some() && !catalog.by_author.contains_key(&material.author));
            let renamed_author = changes.iter().any(|change| match change {
                Change::UpdateUser(user) => {
                    let id = user.id().into_inner();
                    catalog.by_author.contains_key(&id) && users.get(id).is_some_and(|old| old.name() != user.name())
                }
                _ => false,
            });
            new_author || renamed_author
        };
        let _order_guard;
        let mut authors = None;
        if changes_authors {
            _order_guard = self.authors_order.lock().await;
            authors = Some(self.authors_order_from_db(&materials).await);
        }

        let mut users = write(&self.users);
        let mut catalog = write(&self.catalog);
        let mut submissions = write(&self.submissions);
        let mut sessions = write(&self.sessions);
        for change in changes {
            match change {
                Change::InsertUser(user) => users.put(user),
                Change::UpdateUser(mut user) => {
                    // роли бэкенд не пишет: в памяти остаются последние сверенные с БД
                    if let Some(current) = users.get(user.id().into_inner()) {
                        user.sync_roles(current.roles().to_vec());
                    }
                    users.put(user);
                }
                Change::SaveSession(session, expires_at) => {
                    sessions.insert(
                        session.session_id,
                        SessionRec {
                            user_id: session.user_id,
                            verified: session.verified,
                            expires_at,
                        },
                    );
                }
                Change::DeleteSession(id) => {
                    sessions.remove(&id);
                }
                Change::InsertSubmission(submission) | Change::UpdateSubmission(submission) => {
                    submissions.put(sorted_files(submission));
                }
                Change::InsertMaterial(_) => {}
            }
        }
        for material in materials {
            catalog.publish(material);
        }
        if let Some(authors) = authors {
            catalog.authors = authors;
        }
    }

    /// Сверка авторизации с БД: роли пользователей и сессии, удалённые в обход бэкенда.
    ///
    /// Сессии в памяти до запроса уже закоммичены, поэтому если какой-то из них нет в снимке БД — её удалили
    /// (нами или извне), и она убирается. Сессии из БД, которых нет в памяти, не добавляются: это может быть
    /// сессия, которую мы удалили уже после снимка. Роли бэкенд не пишет, значения из БД верны всегда.
    pub async fn sync_auth(&self) -> Result<(), sqlx::Error> {
        let known: HashSet<Uuid> = self.sessions().keys().copied().collect();
        observe_db_query();
        let roles: Vec<(Uuid, Option<Vec<String>>)> = sqlx::query_as("select user_id, roles from web_user")
            .fetch_all(&self.db)
            .await?;
        observe_db_query();
        let alive: HashSet<Uuid> = sqlx::query_scalar("select session_id from session where expires_at > now()")
            .fetch_all(&self.db)
            .await?
            .into_iter()
            .collect();

        let mut changed_roles = 0;
        {
            let mut users = write(&self.users);
            for (id, roles) in roles {
                let roles = roles.unwrap_or_else(|| vec![ROLE_USER.to_owned()]);
                let Some(user) = users.get(id).filter(|user| user.roles() != roles.as_slice()) else {
                    continue;
                };
                let mut user = user.clone();
                user.sync_roles(roles);
                users.put(user);
                changed_roles += 1;
            }
        }
        let revoked: Vec<Uuid> = known.difference(&alive).copied().collect();
        {
            let mut sessions = write(&self.sessions);
            for id in &revoked {
                sessions.remove(id);
            }
        }
        if changed_roles > 0 || !revoked.is_empty() {
            tracing::info!(changed_roles, revoked_sessions = revoked.len(), "synced authorization with the database");
        }
        Ok(())
    }

    /// Записи каталога для новых материалов. Файлы берутся из заявок этого же коммита
    /// (материал публикуется из заявки), недостающие — из БД.
    async fn material_records(&self, changes: &[Change]) -> Vec<MaterialRec> {
        let known: HashMap<Uuid, &MaterialFile> = changes
            .iter()
            .filter_map(|change| match change {
                Change::InsertSubmission(submission) | Change::UpdateSubmission(submission) => Some(submission.files()),
                _ => None,
            })
            .flatten()
            .map(|file| (file.id.into_inner(), file))
            .collect();

        let mut records = Vec::new();
        for change in changes {
            let Change::InsertMaterial(material) = change else {
                continue;
            };
            let ids: Vec<Uuid> = material.file_ids().iter().map(|id| id.into_inner()).collect();
            let mut files: Vec<MaterialFile> = ids.iter().filter_map(|id| known.get(id).map(|file| (*file).clone())).collect();
            if files.len() < ids.len() {
                files = self.files_from_db(&ids).await;
            }
            sort_files(&mut files);
            records.push(MaterialRec::new(
                material.id().into_inner(),
                material.author().into_inner(),
                material.info().clone(),
                Some(material.published_at()),
                material.audit().created(),
                files,
            ));
        }
        records
    }

    async fn files_from_db(&self, ids: &[Uuid]) -> Vec<MaterialFile> {
        observe_db_query();
        let rows: Result<Vec<FileRow>, _> =
            sqlx::query_as(&format!("select {FILE_COLUMNS} from material_file mf where mf.file_id = any($1)"))
                .bind(ids)
                .fetch_all(&self.db)
                .await;
        match rows {
            Ok(rows) => rows.into_iter().map(MaterialFile::from).collect(),
            Err(error) => {
                tracing::error!(%error, "failed to read material files");
                Vec::new()
            }
        }
    }

    /// Авторы (нынешние и новые) в порядке `name, user_id` по правилам сортировки БД.
    async fn authors_order_from_db(&self, new: &[MaterialRec]) -> Vec<Uuid> {
        let current = self.catalog().authors.clone();
        let mut ids = current.clone();
        ids.extend(new.iter().map(|material| material.author).filter(|id| !current.contains(id)));
        ids.dedup();

        observe_db_query();
        let ordered: Result<Vec<Uuid>, _> =
            sqlx::query_scalar("select user_id from web_user where user_id = any($1) order by name, user_id")
                .bind(&ids)
                .fetch_all(&self.db)
                .await;
        match ordered {
            Ok(ordered) => ordered,
            Err(error) => {
                tracing::error!(%error, "failed to order authors, new authors go last");
                ids
            }
        }
    }
}

fn sorted_files(submission: Submission) -> Submission {
    if submission.files().is_sorted_by_key(|file| (file.created, file.id.into_inner())) {
        return submission;
    }
    let mut data = submission.into_data();
    sort_files(&mut data.files);
    Submission::restore(data)
}

async fn linked_files(db: &PgPool, query: &str) -> Result<HashMap<Uuid, Vec<MaterialFile>>, sqlx::Error> {
    let rows: Vec<LinkedFileRow> = sqlx::query_as(&query.replace("{FILE_COLUMNS}", FILE_COLUMNS))
        .fetch_all(db)
        .await?;
    let mut files: HashMap<Uuid, Vec<MaterialFile>> = HashMap::new();
    for row in rows {
        files.entry(row.owner_id).or_default().push(row.file.into());
    }
    Ok(files)
}

async fn sync_auth_periodically(store: std::sync::Weak<Store>) {
    loop {
        tokio::time::sleep(AUTH_SYNC_INTERVAL).await;
        let Some(store) = store.upgrade() else {
            return;
        };
        if let Err(error) = store.sync_auth().await {
            tracing::warn!(%error, "failed to sync authorization with the database");
        }
    }
}

/// Просроченные сессии не находятся сразу; раз в интервал их убираем из памяти и из БД.
async fn clean_sessions(store: std::sync::Weak<Store>) {
    loop {
        tokio::time::sleep(SESSION_CLEANUP_INTERVAL).await;
        let Some(store) = store.upgrade() else {
            return;
        };
        let now = now();
        write(&store.sessions).retain(|_, session| session.expires_at > now);
        observe_db_query();
        if let Err(error) = sqlx::query("delete from session where expires_at < now()")
            .execute(&store.db)
            .await
        {
            tracing::warn!(%error, "failed to delete expired sessions");
        }
    }
}
