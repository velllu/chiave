use axum::{
    body::Body,
    response::{Html, IntoResponse, Response},
};

pub fn error_page(error: &str) -> Response {
    Html(format!("<h1>{}</h1>", error)).into_response()
}

pub fn redirect(url: &str) -> Response {
    Response::builder()
        .status(301)
        .header("Location", url)
        .body(Body::empty())
        .unwrap()
}
