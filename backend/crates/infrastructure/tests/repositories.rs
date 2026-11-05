use chrono::Utc;
use library_domain::materials::{FileName, MaterialFile, MaterialInfo};
use library_domain::submissions::{Submission, SubmissionChanges, SubmissionRepository};
use library_domain::unit_of_work::UnitOfWork;
use library_domain::users::{Email, PasswordHash, User, UserRepository, VerificationCode};
use library_infrastructure::persistence::Db;
use library_infrastructure::persistence::repositories::{SqlxSubmissionRepository, SqlxUserRepository};
use library_infrastructure::persistence::unit_of_work::SqlxUnitOfWork;
use library_test_support::TestDatabase;
use sqlx::PgPool;

fn user(email: &str) -> User {
    User::register(
        Email::parse(email).unwrap(),
        "User".into(),
        PasswordHash::new("hash".into()),
        VerificationCode::generate(),
        Utc::now(),
    )
}

fn info() -> MaterialInfo {
    MaterialInfo {
        title: "Lecture".into(),
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

#[tokio::test]
async fn user_roundtrip_and_unique_email() {
    let (_database, pool) = db().await;
    let users = SqlxUserRepository::new(Db::Pool(pool));
    let user = user("a@example.com");

    assert!(users.insert(&user).await.unwrap());
    assert!(!users.insert(&self::user("a@example.com")).await.unwrap());

    let found = users.find_by_email("a@example.com").await.unwrap().unwrap();
    assert_eq!(found.id(), user.id());
    assert_eq!(found.verification_code(), user.verification_code());
    assert_eq!(found.roles(), ["user"]);
}

#[tokio::test]
async fn submission_files_follow_the_aggregate() {
    let (_database, pool) = db().await;
    let author = user("a@example.com");
    SqlxUserRepository::new(Db::Pool(pool.clone()))
        .insert(&author)
        .await
        .unwrap();
    let submissions = SqlxSubmissionRepository::new(Db::Pool(pool));
    let file = |name: &str| MaterialFile::new(author.id(), FileName::parse(name).unwrap(), 1, None);

    let mut submission = Submission::submit(author.id(), info(), vec![file("a.pdf"), file("b.pdf")], Utc::now());
    submissions.insert(&submission).await.unwrap();
    submission.reject(author.id(), "fix".into(), Utc::now()).unwrap();
    let kept = submission.files()[1].id;
    submission
        .edit(
            SubmissionChanges {
                keep_file_ids: Some(vec![kept]),
                new_files: vec![file("c.pdf")],
                ..Default::default()
            },
            Utc::now(),
        )
        .unwrap();
    assert!(submissions.update(&submission).await.unwrap());

    let stored = submissions.find_by_id(submission.id()).await.unwrap().unwrap();
    let names: Vec<_> = stored.files().iter().map(|file| file.name.as_str()).collect();
    assert_eq!(names, ["b.pdf", "c.pdf"]);
    assert_eq!(stored.status(), submission.status());
}

#[tokio::test]
async fn transaction_without_commit_is_rolled_back() {
    let (_database, pool) = db().await;
    let unit_of_work = SqlxUnitOfWork::new(pool.clone());

    let transaction = unit_of_work.begin().await.unwrap();
    assert!(transaction.users().insert(&user("tx@example.com")).await.unwrap());
    drop(transaction);

    let users = SqlxUserRepository::new(Db::Pool(pool));
    assert!(users.find_by_email("tx@example.com").await.unwrap().is_none());

    let transaction = unit_of_work.begin().await.unwrap();
    transaction.users().insert(&user("tx@example.com")).await.unwrap();
    transaction.commit().await.unwrap();
    assert!(users.find_by_email("tx@example.com").await.unwrap().is_some());
}
