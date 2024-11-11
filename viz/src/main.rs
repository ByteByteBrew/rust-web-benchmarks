use std::net::SocketAddr;

use common::{format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD};
use tokio::net::TcpListener;
use viz::{serve, types::Json, Request, Response, ResponseExt, Result, Router};

async fn plaintext(_: Request) -> Result<&'static str> {
    Ok(HELLO_WORLD)
}

async fn json(_: Request) -> Result<Json<&'static common::JsonResponse<'static>>> {
    Ok(Json(get_json_response()))
}

async fn fortunes(_: Request) -> Result<Response> {
    Ok(Response::html(format_fortunes_html(get_fortunes())))
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await?;
    println!("Server running on http://{addr}");

    let app = Router::new()
        .get("/plaintext", plaintext)
        .get("/json", json)
        .get("/fortunes", fortunes);

    if let Err(e) = serve(listener, app).await {
        println!("Server error: {e}");
    }

    Ok(())
}
