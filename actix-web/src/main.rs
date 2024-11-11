use actix_web::{get, App, HttpResponse, HttpServer};
use common::{
    format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD, HTML_CONTENT_TYPE,
};

#[get("/plaintext")]
async fn plaintext() -> HttpResponse {
    HttpResponse::Ok().body(HELLO_WORLD)
}

#[get("/json")]
async fn json() -> HttpResponse {
    HttpResponse::Ok().json(get_json_response())
}

#[get("/fortunes")]
async fn fortunes() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(HTML_CONTENT_TYPE)
        .body(format_fortunes_html(get_fortunes()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://127.0.0.1:8080");
    HttpServer::new(|| App::new().service(plaintext).service(json).service(fortunes))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
