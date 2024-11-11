use common::{format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD};
use ohkami::{format::JSON, prelude::*, Ohkami};

async fn plaintext() -> &'static str {
    HELLO_WORLD
}

async fn json() -> JSON<&'static common::JsonResponse<'static>> {
    JSON(get_json_response())
}

async fn fortunes() -> Response {
    Response::OK().with_html(format_fortunes_html(get_fortunes()))
}

#[tokio::main]
async fn main() {
    let app =
        Ohkami::new(("/plaintext".GET(plaintext), "/json".GET(json), "/fortunes".GET(fortunes)));

    println!("Server running on http://127.0.0.1:8080");
    app.howl("127.0.0.1:8080").await
}
