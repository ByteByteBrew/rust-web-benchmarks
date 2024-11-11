use common::{
    format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD, HTML_CONTENT_TYPE,
};
use ntex::web::{self, HttpResponse, Responder};

async fn plaintext() -> impl Responder {
    HttpResponse::Ok().body(HELLO_WORLD)
}

async fn json() -> impl Responder {
    HttpResponse::Ok().json(get_json_response())
}

async fn fortunes() -> impl Responder {
    HttpResponse::Ok()
        .content_type(HTML_CONTENT_TYPE)
        .body(format_fortunes_html(get_fortunes()))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://127.0.0.1:8080");
    web::server(|| {
        web::App::new()
            .service(web::resource("/plaintext").route(web::get().to(plaintext)))
            .service(web::resource("/json").route(web::get().to(json)))
            .service(web::resource("/fortunes").route(web::get().to(fortunes)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
