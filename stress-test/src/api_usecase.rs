use rand::prelude::*;
use reqwest::header::AUTHORIZATION;
use serde::de::DeserializeOwned;

const API_URL: &str = "http://localhost:8080";

// this function should imitate use case of api: full user cycle. Registration,
pub async fn api_usecase() -> Result<(), reqwest::Error> {
    let data = MockData::generate();
    let client = reqwest::Client::new();

    let registered: UserBody<User> = send_json(
        client
            .post(format!("{API_URL}/api/users"))
            .json(&serde_json::json!({
                "user": {
                    "username": data.username,
                    "email": data.email,
                    "password": data.password,
                }
            }))
            .send()
            .await?,
    )
    .await?;

    let logged_in: UserBody<User> = send_json(
        client
            .post(format!("{API_URL}/api/users/login"))
            .json(&serde_json::json!({
                "user": {
                    "email": registered.user.email,
                    "password": data.password,
                }
            }))
            .send()
            .await?,
    )
    .await?;

    let token = logged_in.user.token;

    let _current_user: UserBody<User> = send_json(
        client
            .get(format!("{API_URL}/api/user"))
            .header(AUTHORIZATION, format!("Token {token}"))
            .send()
            .await?,
    )
    .await?;

    let _updated_user: UserBody<User> = send_json(
        client
            .put(format!("{API_URL}/api/user"))
            .header(AUTHORIZATION, format!("Token {token}"))
            .json(&serde_json::json!({
                "user": {
                    "bio": &data.about,
                }
            }))
            .send()
            .await?,
    )
    .await?;

    let created_article: ArticleBody<Article> = send_json(
        client
            .post(format!("{API_URL}/api/articles"))
            .header(AUTHORIZATION, format!("Token {token}"))
            .json(&serde_json::json!({
                "article": {
                    "title": &data.article_title,
                    "description": &data.article_description,
                    "body": &data.article_body,
                    "tagList": &data.article_tags,
                }
            }))
            .send()
            .await?,
    )
    .await?;

    let slug = created_article.article.slug;

    send_empty(
        client
            .post(format!("{API_URL}/api/articles/{slug}/favorite"))
            .header(AUTHORIZATION, format!("Token {token}"))
            .send()
            .await?,
    )
    .await?;

    let created_comment: CommentBody<Comment> = send_json(
        client
            .post(format!("{API_URL}/api/articles/{slug}/comments"))
            .header(AUTHORIZATION, format!("Token {token}"))
            .json(&serde_json::json!({
                "comment": { "body": &data.comment_body }
            }))
            .send()
            .await?,
    )
    .await?;

    send_empty(
        client
            .get(format!("{API_URL}/api/articles/{slug}/comments"))
            .send()
            .await?,
    )
    .await?;

    send_empty(
        client
            .delete(format!("{API_URL}/api/articles/{slug}/favorite"))
            .header(AUTHORIZATION, format!("Token {token}"))
            .send()
            .await?,
    )
    .await?;

    let _updated_article: ArticleBody<Article> = send_json(
        client
            .put(format!("{API_URL}/api/articles/{slug}"))
            .header(AUTHORIZATION, format!("Token {token}"))
            .json(&serde_json::json!({
                "article": {
                    "description": &data.article_description_updated,
                    "body": &data.article_body_updated,
                }
            }))
            .send()
            .await?,
    )
    .await?;

    let comment_id = created_comment.comment.id;
    send_empty(
        client
            .delete(format!(
                "{API_URL}/api/articles/{slug}/comments/{comment_id}"
            ))
            .header(AUTHORIZATION, format!("Token {token}"))
            .send()
            .await?,
    )
    .await?;

    send_empty(
        client
            .delete(format!("{API_URL}/api/articles/{slug}"))
            .header(AUTHORIZATION, format!("Token {token}"))
            .send()
            .await?,
    )
    .await?;

    Ok(())
}

struct MockData {
    pub username: String,
    pub email: String,
    pub about: String,
    pub password: String,
    pub article_title: String,
    pub article_description: String,
    pub article_body: String,
    pub article_tags: Vec<String>,
    pub article_description_updated: String,
    pub article_body_updated: String,
    pub comment_body: String,
}

impl MockData {
    pub fn generate() -> Self {
        let title_seed = generate_string(10);
        let article_title = format!("Load test article {title_seed}");
        let article_description = generate_string(32);
        let article_body = format!("{} {}", generate_string(64), generate_string(64));
        let article_tags = vec![
            generate_string(6).to_lowercase(),
            generate_string(6).to_lowercase(),
        ];

        Self {
            username: generate_string(8),
            email: format!("{}@test.com", generate_string(8)),
            about: generate_string(64),
            password: generate_string(12),
            article_title,
            article_description: article_description.clone(),
            article_body: article_body.clone(),
            article_tags,
            article_description_updated: format!("{article_description} updated"),
            article_body_updated: format!("{article_body} updated"),
            comment_body: generate_string(48),
        }
    }
}

fn generate_string(length: usize) -> String {
    rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[derive(serde::Deserialize)]
struct UserBody<T> {
    user: T,
}

#[derive(serde::Deserialize)]
struct User {
    email: String,
    token: String,
}

#[derive(serde::Deserialize)]
struct ArticleBody<T> {
    article: T,
}

#[derive(serde::Deserialize)]
struct Article {
    slug: String,
}

#[derive(serde::Deserialize)]
struct CommentBody<T> {
    comment: T,
}

#[derive(serde::Deserialize)]
struct Comment {
    id: i64,
}

async fn send_json<T: DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T, reqwest::Error> {
    response.error_for_status()?.json().await
}

async fn send_empty(response: reqwest::Response) -> Result<(), reqwest::Error> {
    response.error_for_status().map(|_| ())
}
