use common::{format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD};
use graphul::{
    extract::Json,
    http::{response::Html, Methods},
    Graphul, IntoResponse,
};

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    app.get("/plaintext", || async { HELLO_WORLD });

    app.get("/json", || async { Json(get_json_response()) });

    app.get("/fortunes", || async { Html(format_fortunes_html(get_fortunes())).into_response() });

    println!("Server running on http://127.0.0.1:8080");
    app.run("127.0.0.1:8080").await;
}
