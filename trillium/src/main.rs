use common::{
    format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD, HTML_CONTENT_TYPE,
};
use trillium::Conn;
use trillium_router::Router;

async fn plaintext(conn: Conn) -> Conn {
    conn.ok(HELLO_WORLD)
}

async fn json(conn: Conn) -> Conn {
    conn.with_response_header("content-type", "application/json")
        .ok(serde_json::to_string(&get_json_response()).unwrap())
}

async fn fortunes(conn: Conn) -> Conn {
    conn.with_response_header("content-type", HTML_CONTENT_TYPE)
        .ok(format_fortunes_html(get_fortunes()))
}

fn main() {
    let app = Router::new()
        .get("/plaintext", plaintext)
        .get("/json", json)
        .get("/fortunes", fortunes);

    println!("Server running on http://0.0.0.0:8080");
    trillium_smol::run(app);
}
