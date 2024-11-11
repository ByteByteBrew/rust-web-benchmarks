use common::{format_fortunes_html, get_fortunes, get_json_response, JsonResponse, HELLO_WORLD};
use rocket::{get, launch, routes, serde::json::Json};

#[get("/plaintext")]
fn plaintext() -> &'static str {
    HELLO_WORLD
}

#[get("/json")]
fn json() -> Json<&'static JsonResponse<'static>> {
    Json(get_json_response())
}

#[get("/fortunes")]
fn fortunes() -> (rocket::http::ContentType, String) {
    (rocket::http::ContentType::HTML, format_fortunes_html(get_fortunes()))
}

#[launch]
fn rocket() -> _ {
    let config = rocket::Config::figment().merge(("port", 8080));

    println!("Server running on http://127.0.0.1:8080");
    rocket::custom(config).mount("/", routes![plaintext, json, fortunes])
}
