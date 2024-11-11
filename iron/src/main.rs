use common::{
    format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD, HTML_CONTENT_TYPE,
};
use iron::{mime::Mime, prelude::*, status};
use router::Router;

fn plaintext(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, HELLO_WORLD)))
}

fn json(_: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    let json_str = serde_json::to_string(get_json_response()).unwrap();
    let mut response = Response::with((status::Ok, json_str));
    response.headers.set(iron::headers::ContentType(content_type));
    Ok(response)
}

fn fortunes(_: &mut Request) -> IronResult<Response> {
    let content_type = HTML_CONTENT_TYPE.parse::<Mime>().unwrap();
    let mut response = Response::with((status::Ok, format_fortunes_html(get_fortunes())));
    response.headers.set(iron::headers::ContentType(content_type));
    Ok(response)
}

fn main() {
    let mut router = Router::new();
    router.get("/plaintext", plaintext, "plaintext");
    router.get("/json", json, "json");
    router.get("/fortunes", fortunes, "fortunes");

    println!("Server running on http://127.0.0.1:8080");
    Iron::new(router).http("127.0.0.1:8080").unwrap();
}
