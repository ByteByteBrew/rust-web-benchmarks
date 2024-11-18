use common::{format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD};
use summer_boot::{Request, Response, Result, StatusCode};

#[summer_boot::auto_scan()]
#[summer_boot::main]
async fn main() {
    let mut app = summer_boot::new();
    app.at("/plaintext").get(plaintext);
    app.at("/json").get(json);
    app.at("/fortunes").get(fortunes);
    app.listen("127.0.0.1:8080").await.unwrap();
}

#[summer_boot::get("/plaintext")]
pub async fn plaintext(_req: Request<()>) -> Result {
    Ok(HELLO_WORLD.into())
}

#[summer_boot::get("/json")]
pub async fn json(_req: Request<()>) -> Result {
    let mut resp = Response::new(StatusCode::Ok);
    resp.set_content_type("application/json");
    resp.body_json(&get_json_response())?;
    Ok(resp)
}

#[summer_boot::get("/fortunes")]
pub async fn fortunes(_req: Request<()>) -> Result {
    let mut resp = Response::new(StatusCode::Ok);
    resp.set_content_type("text/html");
    resp.body_string(format_fortunes_html(get_fortunes()));
    Ok(resp)
}
