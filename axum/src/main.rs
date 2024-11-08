use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use common::{
    format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD, HTML_CONTENT_TYPE,
};

async fn plaintext() -> impl IntoResponse {
    (StatusCode::OK, HELLO_WORLD)
}

async fn json() -> impl IntoResponse {
    (StatusCode::OK, Json(get_json_response()))
}

async fn fortunes() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, HTML_CONTENT_TYPE)],
        format_fortunes_html(get_fortunes()),
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/plaintext", get(plaintext))
        .route("/json", get(json))
        .route("/fortunes", get(fortunes));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running on http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
