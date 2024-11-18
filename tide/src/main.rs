use common::{
    format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD, HTML_CONTENT_TYPE,
};
use tide::{Request, Response, StatusCode};

async fn plaintext(_: Request<()>) -> tide::Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(HELLO_WORLD);
    Ok(res)
}

async fn json(_: Request<()>) -> tide::Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(tide::Body::from_json(&get_json_response())?);
    Ok(res)
}

async fn fortunes(_: Request<()>) -> tide::Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_content_type(HTML_CONTENT_TYPE);
    res.set_body(format_fortunes_html(get_fortunes()));
    Ok(res)
}

#[tokio::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/plaintext").get(plaintext);
    app.at("/json").get(json);
    app.at("/fortunes").get(fortunes);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
