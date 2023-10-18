use std::env;

use axum::{body::Body, http::Response, response::Html, Form};
use serde::Deserialize;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

#[derive(Deserialize, Debug)]
pub struct UserData {
    username: String,
    password: String,
}

async fn get_pool() -> Pool<Sqlite> {
    SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap()
}

pub async fn login(Form(user_data): Form<UserData>) -> Html<String> {
    // TODO: Check for auth cookie

    let connection = get_pool().await;

    let user = sqlx::query!(
        r#"
        SELECT username, password FROM users
        WHERE username = $1 AND password = $2
        "#,
        user_data.username,
        user_data.password
    )
    .fetch_optional(&connection)
    .await
    .unwrap();

    if user.is_some() {
        todo!()
    } else {
        todo!()
    }
}

pub async fn signup(Form(user_data): Form<UserData>) -> Response<Body> {
    let connection = get_pool().await;

    let _ = sqlx::query!(
        "INSERT INTO users (username, password) VALUES ($1, $2)",
        user_data.username,
        user_data.password, // TODO: Add sha512 encryption
    )
    .execute(&connection)
    .await;

    Response::builder()
        .status(301)
        .header("Location", "www.google.com")
        .body(Body::empty())
        .unwrap()
}
