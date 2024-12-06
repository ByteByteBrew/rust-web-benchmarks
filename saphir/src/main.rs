use common::{
    format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD, HTML_CONTENT_TYPE,
};
use saphir::{prelude::*, responder::Responder};
use serde_json::Value;

struct HtmlResponse(String);

impl Responder for HtmlResponse {
    fn respond_with_builder(self, builder: Builder, _ctx: &HttpContext) -> Builder {
        builder.status(200).body(self.0).header("content-type", HTML_CONTENT_TYPE)
    }
}

async fn plaintext(_: Request) -> (u16, &'static str) {
    (200, HELLO_WORLD)
}

async fn json(_: Request) -> Json<Value> {
    let response = get_json_response();
    Json(serde_json::to_value(response).unwrap())
}

async fn fortunes(_: Request) -> HtmlResponse {
    HtmlResponse(format_fortunes_html(get_fortunes()))
}

#[tokio::main]
async fn main() -> Result<(), SaphirError> {
    let server = Server::builder()
        .configure_listener(|l| l.interface("0.0.0.0:8080"))
        .configure_router(|r| {
            r.route("/plaintext", Method::GET, plaintext)
                .route("/json", Method::GET, json)
                .route("/fortunes", Method::GET, fortunes)
        })
        .build();

    server.run().await?;
    Ok(())
}
