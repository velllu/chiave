use askama::Template;
use axum::{response::Html, routing::get, Router};

mod api;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/login", get(login).post(api::login))
        .route("/signup", get(signup).post(api::signup));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// TODO: Handle errors
async fn login() -> Html<String> {
    Html(Login.render().unwrap())
}

async fn signup() -> Html<String> {
    Html(Signup.render().unwrap())
}

#[derive(Template)]
#[template(path = "login.html")]
struct Login;

#[derive(Template)]
#[template(path = "signup.html")]
struct Signup;
