use common::{format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD};
use gotham::{
    handler::HandlerResult,
    helpers::http::response::create_response,
    hyper::StatusCode,
    mime::{APPLICATION_JSON, TEXT_HTML, TEXT_PLAIN},
    prelude::{DefineSingleRoute, DrawRoutes},
    state::State,
};

async fn plaintext(state: State) -> HandlerResult {
    let res = create_response(&state, StatusCode::OK, TEXT_PLAIN, HELLO_WORLD);
    Ok((state, res))
}

async fn json(state: State) -> HandlerResult {
    let body = serde_json::to_string(get_json_response()).unwrap();
    let res = create_response(&state, StatusCode::OK, APPLICATION_JSON, body);
    Ok((state, res))
}

async fn fortunes(state: State) -> HandlerResult {
    let body = format_fortunes_html(get_fortunes());
    let res = create_response(&state, StatusCode::OK, TEXT_HTML, body);
    Ok((state, res))
}

fn router() -> gotham::router::Router {
    gotham::router::builder::build_simple_router(|route| {
        route.get("/plaintext").to_async(plaintext);
        route.get("/json").to_async(json);
        route.get("/fortunes").to_async(fortunes);
    })
}

fn main() {
    let addr = "127.0.0.1:8080";
    println!("Server running on http://{addr}");
    gotham::start(addr, router()).unwrap();
}
