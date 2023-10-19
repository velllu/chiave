use std::env;

use axum::{response::Response, Form};
use serde::Deserialize;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use crate::pages::{error_page, redirect};

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

pub async fn login(Form(user_data): Form<UserData>) -> Result<Response, Response> {
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
        Ok(redirect("https://www.google.com/"))
    } else {
        Err(error_page("User was not found"))
    }
}

pub async fn signup(Form(user_data): Form<UserData>) -> Result<Response, Response> {
    if user_data.username.len() >= 15 {
        return Err(error_page("Username must be under 15 characters"));
    }

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
        return Err(error_page("User already exists"));
    }

    let _ = sqlx::query!(
        "INSERT INTO users (username, password) VALUES ($1, $2)",
        user_data.username,
        user_data.password, // TODO: Add sha512 encryption
    )
    .execute(&connection)
    .await;

    Ok(redirect("/"))
}
