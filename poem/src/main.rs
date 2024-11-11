use common::{format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD};
use poem::{
    get, handler,
    listener::TcpListener,
    web::{Html, Json},
    Route, Server,
};

#[handler]
async fn plaintext() -> String {
    HELLO_WORLD.to_string()
}

#[handler]
async fn json() -> Json<&'static common::JsonResponse<'static>> {
    Json(get_json_response())
}

#[handler]
async fn fortunes() -> Html<String> {
    Html(format_fortunes_html(get_fortunes()))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/plaintext", get(plaintext))
        .at("/json", get(json))
        .at("/fortunes", get(fortunes));

    println!("Server running on http://127.0.0.1:8080");
    Server::new(TcpListener::bind("127.0.0.1:8080")).run(app).await
}
