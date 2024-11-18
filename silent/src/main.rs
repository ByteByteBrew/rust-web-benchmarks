use common::{format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD};
use silent::{
    header::{HeaderValue, CONTENT_TYPE},
    prelude::*,
};

async fn plaintext(_req: Request) -> Result<Response> {
    let mut res = Response::empty();
    res.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    res.set_body(full(HELLO_WORLD));
    Ok(res)
}

async fn json(_req: Request) -> Result<Response> {
    let mut res = Response::empty();
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    res.set_body(full(serde_json::to_string(&get_json_response())?));
    Ok(res)
}

async fn fortunes(_req: Request) -> Result<Response> {
    let mut res = Response::empty();
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/html; charset=utf-8"));
    res.set_body(full(format_fortunes_html(get_fortunes())));
    Ok(res)
}

fn main() {
    let route = Route::new("")
        .append(Route::new("plaintext").get(plaintext))
        .append(Route::new("json").get(json))
        .append(Route::new("fortunes").get(fortunes));

    Server::new().bind("127.0.0.1:8080".parse().unwrap()).run(route);
}
