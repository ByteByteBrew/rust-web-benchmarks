use common::{format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD};
use xitca_web::{
    body::ResponseBody,
    handler::handler_service,
    http::{header, HeaderValue, Response},
    route::get,
    App,
};

async fn plaintext() -> &'static str {
    HELLO_WORLD
}

async fn json() -> Response<ResponseBody> {
    let mut response =
        Response::new(ResponseBody::from(serde_json::to_string(get_json_response()).unwrap()));
    response
        .headers_mut()
        .insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
    response
}

async fn fortunes() -> Response<ResponseBody> {
    let mut response = Response::new(ResponseBody::from(format_fortunes_html(get_fortunes())));
    response
        .headers_mut()
        .insert(header::CONTENT_TYPE, HeaderValue::from_static("text/html; charset=utf-8"));
    response
}

fn main() -> std::io::Result<()> {
    println!("Server running on http://127.0.0.1:8080");

    App::new()
        .at("/plaintext", get(handler_service(plaintext)))
        .at("/json", get(handler_service(json)))
        .at("/fortunes", get(handler_service(fortunes)))
        .serve()
        .bind("127.0.0.1:8080")?
        .run()
        .wait()
}
