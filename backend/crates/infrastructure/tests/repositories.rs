//! Хранилище в памяти: всё записанное должно оказаться в БД ровно в том же виде —
//! проверяем, загружая из БД свежее хранилище и сравнивая сущности целиком.

use std::sync::Arc;

use chrono::TimeDelta;
use library_application::auth::Session;
use library_application::ports::SessionStore;

use library_domain::common::{PageRequest, now};
use library_domain::materials::{FileName, MaterialFile, MaterialId, MaterialInfo, MaterialRepository};
use library_domain::submissions::{Submission, SubmissionChanges, SubmissionRepository, SubmissionStatus};
use library_domain::unit_of_work::UnitOfWork;
use library_domain::users::{Email, PasswordHash, User, UserRepository, VerificationCode};
use library_infrastructure::persistence::Store;
use library_infrastructure::persistence::repositories::{
    MemoryMaterialRepository, MemorySessionStore, MemorySubmissionRepository, MemoryUnitOfWork, MemoryUserRepository,
};
use library_test_support::TestDatabase;
use sqlx::PgPool;
use uuid::Uuid;

fn user(email: &str) -> User {
    User::register(
        Email::parse(email).unwrap(),
        "User".into(),
        PasswordHash::new("hash".into()),
        VerificationCode::generate(),
        now(),
    )
}

fn info(title: &str) -> MaterialInfo {
    MaterialInfo {
        title: title.into(),
        description: String::new(),
        courses: vec!["1".into()],
        subjects: vec!["math".into()],
        kind: "notes".into(),
        difficulty: "none".into(),
    }
}

async fn db() -> (TestDatabase, PgPool) {
    let database = TestDatabase::create().await;
    let pool = database.connect().await;
    (database, pool)
}

/// Свежее хранилище, загруженное из БД.
async fn reload(pool: &PgPool) -> Arc<Store> {
    Store::load(pool.clone()).await.unwrap()
}

#[tokio::test]
async fn user_roundtrip_and_unique_email() {
    let (_database, pool) = db().await;
    let users = MemoryUserRepository::new(reload(&pool).await);
    let user = user("a@example.com");

    assert!(users.insert(&user).await.unwrap());
    assert!(!users.insert(&self::user("a@example.com")).await.unwrap());
    assert_eq!(
        users.find_by_email("a@example.com").await.unwrap().as_ref(),
        Some(&user)
    );

    let reloaded = MemoryUserRepository::new(reload(&pool).await);
    assert_eq!(
        reloaded.find_by_email("a@example.com").await.unwrap(),
        Some(user.clone())
    );
}

#[tokio::test]
async fn external_role_and_session_changes_are_synced() {
    let (_database, pool) = db().await;
    let store = reload(&pool).await;
    let users = MemoryUserRepository::new(store.clone());
    let sessions = MemorySessionStore::new(store.clone());
    let user = user("a@example.com");
    users.insert(&user).await.unwrap();
    let session = |n: u128| Session {
        user_id: user.id().into_inner(),
        session_id: Uuid::from_u128(n),
        verified: true,
    };
    for n in [1, 2] {
        sessions.save(&session(n), TimeDelta::hours(1)).await.unwrap();
    }

    sqlx::query("update web_user set roles = array['user', 'moderator']")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("delete from session where session_id = $1")
        .bind(Uuid::from_u128(1))
        .execute(&pool)
        .await
        .unwrap();
    assert!(!users.find_by_id(user.id()).await.unwrap().unwrap().is_moderator());

    store.sync_auth().await.unwrap();

    assert!(users.find_by_id(user.id()).await.unwrap().unwrap().is_moderator());
    assert!(sessions.find(Uuid::from_u128(1)).await.unwrap().is_none());
    assert!(sessions.find(Uuid::from_u128(2)).await.unwrap().is_some());

    // своя правка пользователя не затирает сверенные роли
    let mut edited = users.find_by_id(user.id()).await.unwrap().unwrap();
    edited.update_profile(None, Some("bio".into()));
    users.update(&edited).await.unwrap();
    let roles: Vec<String> = sqlx::query_scalar("select roles from web_user")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(roles, ["user", "moderator"]);
    assert!(users.find_by_id(user.id()).await.unwrap().unwrap().is_moderator());
}

#[tokio::test]
async fn concurrent_writes_are_all_committed() {
    let (_database, pool) = db().await;
    let users = Arc::new(MemoryUserRepository::new(reload(&pool).await));

    let handles: Vec<_> = (0..64)
        .map(|n| {
            let users = users.clone();
            tokio::spawn(async move { users.insert(&user(&format!("u{n}@example.com"))).await.unwrap() })
        })
        .collect();
    for handle in handles {
        assert!(handle.await.unwrap());
    }

    let count: i64 = sqlx::query_scalar("select count(*) from web_user")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 64);
}

#[tokio::test]
async fn submission_files_follow_the_aggregate() {
    let (_database, pool) = db().await;
    let store = reload(&pool).await;
    let author = user("a@example.com");
    MemoryUserRepository::new(store.clone()).insert(&author).await.unwrap();
    let submissions = MemorySubmissionRepository::new(store);
    let file = |name: &str| MaterialFile::new(author.id(), FileName::parse(name).unwrap(), 1, None);

    let mut submission = Submission::submit(author.id(), info("Lecture"), vec![file("a.pdf"), file("b.pdf")], now());
    submissions.insert(&submission).await.unwrap();
    submission.reject(author.id(), "fix".into(), now()).unwrap();
    let kept = submission.files()[1].id;
    submission
        .edit(
            SubmissionChanges {
                keep_file_ids: Some(vec![kept]),
                new_files: vec![file("c.pdf")],
                ..Default::default()
            },
            now(),
        )
        .unwrap();
    assert!(submissions.update(&submission).await.unwrap());

    let reloaded = MemorySubmissionRepository::new(reload(&pool).await);
    let stored = reloaded.find_by_id(submission.id()).await.unwrap().unwrap();
    let names: Vec<_> = stored.files().iter().map(|file| file.name.as_str()).collect();
    assert_eq!(names, ["b.pdf", "c.pdf"]);
    assert_eq!(stored, submission);

    let own = reloaded
        .list_by_author(
            author.id(),
            Some(SubmissionStatus::PendingReview),
            PageRequest::default(),
        )
        .await
        .unwrap();
    assert_eq!((own.total, own.items.len()), (1, 1));
    assert_eq!(reloaded.count_by_status().await.unwrap().pending_review, 1);
}

#[tokio::test]
async fn transaction_publishes_material_atomically() {
    let (_database, pool) = db().await;
    let store = reload(&pool).await;
    let author = user("a@example.com");
    MemoryUserRepository::new(store.clone()).insert(&author).await.unwrap();
    let file = MaterialFile::new(author.id(), FileName::parse("a.pdf").unwrap(), 1, None);
    let mut submission = Submission::submit(author.id(), info("Lecture"), vec![file], now());
    MemorySubmissionRepository::new(store.clone())
        .insert(&submission)
        .await
        .unwrap();
    let unit_of_work = MemoryUnitOfWork::new(store.clone());
    let materials = MemoryMaterialRepository::new(store.clone());

    let material = submission.clone().approve(author.id(), now()).unwrap();
    let transaction = unit_of_work.begin().await.unwrap();
    transaction.materials().insert(&material).await.unwrap();
    drop(transaction);
    assert!(materials.find_by_id(material.id()).await.unwrap().is_none());

    let material = submission.approve(author.id(), now()).unwrap();
    let transaction = unit_of_work.begin().await.unwrap();
    transaction.materials().insert(&material).await.unwrap();
    assert!(transaction.submissions().update(&submission).await.unwrap());
    transaction.commit().await.unwrap();

    for materials in [materials, MemoryMaterialRepository::new(reload(&pool).await)] {
        let found = materials.find_by_id(material.id()).await.unwrap().unwrap();
        assert_eq!(
            (found.author_name.as_str(), found.published_at),
            ("User", Some(material.published_at()))
        );
        assert_eq!(
            materials
                .files(MaterialId::from(material.id().into_inner()))
                .await
                .unwrap(),
            submission.files()
        );
        let feed = materials
            .list_published(Some("LECT".into()), PageRequest::default())
            .await
            .unwrap();
        assert_eq!(feed.total, 1);
    }
    let authors = MemoryUserRepository::new(store)
        .list_authors(None, PageRequest::default())
        .await
        .unwrap();
    assert_eq!((authors.total, authors.items[0].materials_count), (1, 1));
}
