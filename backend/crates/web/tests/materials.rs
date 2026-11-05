mod common;

use axum::http::{Method, StatusCode};
use common::TestApp;
use serde_json::json;

#[tokio::test]
async fn submission_goes_through_moderation_to_catalogue() {
    let app = TestApp::spawn().await;
    let (author_id, author, _) = app.verified_user("author@example.com").await;
    let moderator = app.moderator("mod@example.com").await;

    let created = app.submit(&author, "Linear algebra", &["la.pdf", "la.pdf"]).await;
    assert_eq!(created.status, StatusCode::OK, "{}", created.body);
    assert_eq!(created.body["status"], "pending_review");
    assert_eq!(created.body["files"].as_array().unwrap().len(), 2);
    assert_eq!(
        app.storage.0.lock().unwrap().len(),
        2,
        "same names must not overwrite each other"
    );
    let id = created.body["id"].as_str().unwrap().to_owned();

    let queue = app.get("/api/moderation/submissions", Some(&moderator)).await;
    assert_eq!(queue.status, StatusCode::OK);
    assert_eq!(queue.body["total"], 1);
    assert_eq!(queue.body["counters"]["pending_review"], 1);
    assert_eq!(queue.body["items"][0]["id"], id.as_str());

    let decision = app
        .post(
            &format!("/api/moderation/submissions/{id}/decision"),
            Some(&moderator),
            json!({"action": "approve"}),
        )
        .await;
    assert_eq!(decision.status, StatusCode::OK, "{}", decision.body);
    assert_eq!(decision.body["status"], "approved");
    assert!(decision.body["published_at"].is_string());

    let catalogue = app.get("/api/materials?search=algebra", None).await;
    assert_eq!(catalogue.body["total"], 1);
    let material = &catalogue.body["items"][0];
    assert_eq!(material["title"], "Linear algebra");
    assert_eq!(material["type"], "notes");
    assert_eq!(material["author_name"], "User author@example.com");

    let details = app
        .get(&format!("/api/materials/{}", material["id"].as_str().unwrap()), None)
        .await;
    assert_eq!(details.status, StatusCode::OK);
    assert_eq!(details.body["files"].as_array().unwrap().len(), 2);
    assert_eq!(details.body["files"][0]["extension"], "pdf");

    let authors = app.get("/api/users", None).await;
    assert_eq!(authors.body["total"], 1);
    assert_eq!(authors.body["items"][0]["materials_count"], 1);

    let profile = app.get(&format!("/api/users/{author_id}"), None).await;
    assert_eq!(profile.body["user"]["materials_count"], 1);
    assert_eq!(profile.body["materials"]["items"][0]["title"], "Linear algebra");
}

#[tokio::test]
async fn rejected_submission_can_be_edited_with_new_and_kept_files() {
    let app = TestApp::spawn().await;
    let (_, author, _) = app.verified_user("author@example.com").await;
    let moderator = app.moderator("mod@example.com").await;
    let created = app.submit(&author, "Draft", &["a.pdf", "b.pdf"]).await;
    let id = created.body["id"].as_str().unwrap().to_owned();
    let kept = created.body["files"][0]["id"].as_str().unwrap().to_owned();

    let no_comment = app
        .post(
            &format!("/api/moderation/submissions/{id}/decision"),
            Some(&moderator),
            json!({"action": "reject"}),
        )
        .await;
    assert_eq!(no_comment.status, StatusCode::BAD_REQUEST);
    let rejected = app
        .post(
            &format!("/api/moderation/submissions/{id}/decision"),
            Some(&moderator),
            json!({"action": "reject", "moderator_comment": "add more"}),
        )
        .await;
    assert_eq!(rejected.body["status"], "rejected");
    assert_eq!(rejected.body["moderator_comment"], "add more");

    let keep = json!([kept]).to_string();
    let updated = app
        .multipart(
            Method::PATCH,
            &format!("/api/materials/submissions/{id}"),
            &author,
            &[
                ("title", None, None, b"Final"),
                ("courses", None, None, b"[]"),
                ("keepFileIds", None, None, keep.as_bytes()),
                ("files", Some("c.py"), Some("text/x-python"), b"print(1)"),
            ],
        )
        .await;
    assert_eq!(updated.status, StatusCode::OK, "{}", updated.body);
    assert_eq!(updated.body["status"], "pending_review");
    let names: Vec<_> = updated.body["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|file| file["name"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(names, ["a.pdf", "c.py"]);

    let fetched = app
        .get(&format!("/api/materials/submissions/{id}"), Some(&author))
        .await;
    assert_eq!(fetched.body["files"].as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn submission_access_rules() {
    let app = TestApp::spawn().await;
    let (_, author, _) = app.verified_user("author@example.com").await;
    let (_, stranger, _) = app.verified_user("stranger@example.com").await;
    let moderator = app.moderator("mod@example.com").await;
    let id = app.submit(&author, "Mine", &[]).await.body["id"]
        .as_str()
        .unwrap()
        .to_owned();
    let uri = format!("/api/materials/submissions/{id}");

    assert_eq!(app.get(&uri, Some(&stranger)).await.status, StatusCode::FORBIDDEN);
    assert_eq!(app.get(&uri, Some(&moderator)).await.status, StatusCode::OK);
    assert_eq!(app.get(&uri, Some(&author)).await.status, StatusCode::OK);
    // на модерации править нельзя
    let edit = app
        .multipart(Method::PATCH, &uri, &author, &[("title", None, None, b"New")])
        .await;
    assert_eq!(edit.status, StatusCode::FORBIDDEN);
    assert_eq!(
        app.get("/api/moderation/submissions", Some(&stranger)).await.status,
        StatusCode::FORBIDDEN
    );
    let missing = app
        .get(
            &format!("/api/materials/submissions/{}", uuid::Uuid::now_v7()),
            Some(&author),
        )
        .await;
    assert_eq!(missing.status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn own_submissions_are_filtered_by_status() {
    let app = TestApp::spawn().await;
    let (_, author, _) = app.verified_user("author@example.com").await;
    app.submit(&author, "One", &[]).await;
    app.submit(&author, "Two", &[]).await;

    let all = app.get("/api/materials/submissions?status=all", Some(&author)).await;
    let rejected = app
        .get("/api/materials/submissions?status=rejected", Some(&author))
        .await;
    let unknown = app.get("/api/materials/submissions?status=nope", Some(&author)).await;

    assert_eq!(all.body["total"], 2);
    assert_eq!(all.body["items"].as_array().unwrap().len(), 2);
    assert_eq!(rejected.body["total"], 0);
    assert!(rejected.body["items"].as_array().unwrap().is_empty());
    assert_eq!(unknown.status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn invalid_uploads_are_rejected() {
    let app = TestApp::spawn().await;
    let (_, author, _) = app.verified_user("author@example.com").await;
    app.register("unverified@example.com").await;

    let bad_extension = app.submit(&author, "Bad", &["virus.exe"]).await;
    let no_data = app
        .multipart(
            Method::POST,
            "/api/materials/submissions",
            &author,
            &[("files", Some("a.pdf"), Some("application/pdf"), b"x")],
        )
        .await;

    assert_eq!(bad_extension.status, StatusCode::BAD_REQUEST);
    assert_eq!(bad_extension.body["details"][0]["field"], "files");
    assert_eq!(no_data.status, StatusCode::BAD_REQUEST);
    assert!(app.storage.0.lock().unwrap().is_empty());
}
